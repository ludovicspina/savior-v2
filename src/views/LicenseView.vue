<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { ExternalLink, KeyRound, RefreshCw } from "lucide-vue-next";
import { useUiStore } from "@/stores/ui";
import type { WindowsLicenseInfo } from "@/types/license";

const ui = useUiStore();

const info = ref<WindowsLicenseInfo | null>(null);
const loading = ref(false);
const error = ref<string | null>(null);
const actionMessage = ref<string | null>(null);
const actionLoading = ref(false);

const isWindows = computed(() => ui.currentOs === "windows");

const primaryProduct = computed(() => {
  const products = info.value?.products ?? [];
  return products.find((p) => p.licenseStatus === 1) ?? products[0] ?? null;
});

function statusColor(activated: boolean): string {
  return activated ? "var(--status-ok)" : "var(--status-warn)";
}

async function fetchStatus() {
  if (!isWindows.value) return;
  loading.value = true;
  error.value = null;
  try {
    info.value = await invoke<WindowsLicenseInfo>("windows_license_status");
  } catch (err) {
    error.value = err instanceof Error ? err.message : String(err);
    info.value = null;
  } finally {
    loading.value = false;
  }
}

async function openSettings() {
  actionMessage.value = null;
  actionLoading.value = true;
  try {
    await invoke("windows_open_activation_settings");
    actionMessage.value = "Paramètres d'activation ouverts.";
  } catch (err) {
    actionMessage.value = err instanceof Error ? err.message : String(err);
  } finally {
    actionLoading.value = false;
  }
}

async function runPlaceholderCommand() {
  actionMessage.value = null;
  actionLoading.value = true;
  try {
    const output = await invoke<string>("windows_license_placeholder_command");
    actionMessage.value = output;
  } catch (err) {
    actionMessage.value = err instanceof Error ? err.message : String(err);
  } finally {
    actionLoading.value = false;
  }
}

onMounted(() => {
  if (isWindows.value) fetchStatus();
});
</script>

<template>
  <div class="mx-auto max-w-3xl space-y-5">
    <div class="flex flex-wrap items-start justify-between gap-3">
      <div>
        <h1 class="flex items-center gap-2 text-xl font-semibold">
          <KeyRound :size="22" :stroke-width="1.75" class="text-[var(--accent-strong)]" />
          Licence Windows
        </h1>
        <p class="mt-1 text-sm text-[var(--text-muted)]">
          État d'activation, dépannage et raccourcis système.
        </p>
      </div>
      <button
        v-if="isWindows"
        type="button"
        class="inline-flex items-center gap-2 rounded-lg border border-[var(--border)] px-3 py-2 text-sm text-[var(--text-muted)] hover:bg-[var(--surface-2)] hover:text-[var(--text)] disabled:opacity-50"
        :disabled="loading"
        @click="fetchStatus"
      >
        <RefreshCw :size="16" :class="loading ? 'animate-spin' : ''" />
        Actualiser
      </button>
    </div>

    <div
      v-if="!isWindows"
      class="card p-4 text-sm text-[var(--text-muted)]"
    >
      Module disponible uniquement sur Windows.
    </div>

    <template v-else>
      <div v-if="loading && !info" class="card p-6 text-sm text-[var(--text-muted)]">
        Lecture de l'état d'activation…
      </div>

      <div v-else-if="error" class="card border-[var(--status-crit)] p-4 text-sm text-[var(--status-crit)]">
        {{ error }}
      </div>

      <div v-else-if="info" class="card overflow-hidden">
        <div class="card-header">Résumé</div>
        <div class="flex flex-wrap items-center justify-between gap-4 p-4">
          <div>
            <p class="text-xs uppercase tracking-wide text-[var(--text-faint)]">Statut</p>
            <p
              class="mt-1 text-lg font-semibold"
              :style="{ color: statusColor(info.activated) }"
            >
              {{ info.activated ? "Activé" : "Non activé ou en période de grâce" }}
            </p>
            <p v-if="primaryProduct" class="mt-1 text-sm text-[var(--text-muted)]">
              {{ primaryProduct.description || primaryProduct.name }}
              <span v-if="primaryProduct.partialProductKey" class="font-tabular">
                · {{ primaryProduct.partialProductKey }}
              </span>
            </p>
          </div>
          <div
            class="rounded-full px-3 py-1 text-xs font-semibold uppercase tracking-wide"
            :style="{
              background: info.activated ? 'color-mix(in srgb, var(--status-ok) 15%, transparent)' : 'color-mix(in srgb, var(--status-warn) 15%, transparent)',
              color: statusColor(info.activated),
            }"
          >
            {{ primaryProduct?.statusLabel ?? "Inconnu" }}
          </div>
        </div>
      </div>

      <div v-if="info?.products.length" class="card overflow-hidden">
        <div class="card-header">Produits détectés</div>
        <div class="divide-y divide-[var(--border-muted)]">
          <div
            v-for="(product, index) in info.products"
            :key="`${product.name}-${index}`"
            class="grid gap-2 p-4 text-sm sm:grid-cols-[1fr_auto]"
          >
            <div>
              <p class="font-medium">{{ product.description || product.name }}</p>
              <p class="mt-0.5 text-xs text-[var(--text-muted)]">{{ product.name }}</p>
            </div>
            <div class="text-right text-xs text-[var(--text-muted)] sm:text-sm">
              <p :style="{ color: product.licenseStatus === 1 ? 'var(--status-ok)' : 'var(--text)' }">
                {{ product.statusLabel }}
              </p>
              <p v-if="product.partialProductKey" class="mt-1 font-tabular">
                Clé partielle : {{ product.partialProductKey }}
              </p>
              <p v-if="product.gracePeriodRemaining" class="mt-1">
                Grâce restante : {{ product.gracePeriodRemaining }} min
              </p>
            </div>
          </div>
        </div>
      </div>

      <div class="card overflow-hidden">
        <div class="card-header">Actions</div>
        <div class="flex flex-col gap-3 p-4 sm:flex-row sm:flex-wrap">
          <button
            type="button"
            class="inline-flex items-center justify-center gap-2 rounded-lg bg-[var(--accent)] px-4 py-2 text-sm font-medium text-[#0c0f14] hover:opacity-90 disabled:opacity-50"
            :disabled="actionLoading"
            @click="openSettings"
          >
            <ExternalLink :size="16" />
            Ouvrir les paramètres d'activation
          </button>
          <button
            type="button"
            class="rounded-lg border border-[var(--border)] px-4 py-2 text-sm text-[var(--text-muted)] hover:bg-[var(--surface-2)] hover:text-[var(--text)] disabled:opacity-50"
            :disabled="actionLoading"
            @click="runPlaceholderCommand"
          >
            Commande PowerShell
          </button>
        </div>
        <p
          v-if="actionMessage"
          class="border-t border-[var(--border-muted)] px-4 py-3 font-tabular text-sm text-[var(--text-muted)]"
        >
          {{ actionMessage }}
        </p>
      </div>
    </template>
  </div>
</template>

export interface LicenseProduct {
  name: string;
  description: string;
  licenseStatus: number;
  statusLabel: string;
  partialProductKey: string | null;
  gracePeriodRemaining: number;
}

export interface WindowsLicenseInfo {
  activated: boolean;
  products: LicenseProduct[];
}

const RELEASE_BASE =
  "https://github.com/russellmcc/bilinear-audio/releases/latest/download";

export type ProductDownloads = {
  macUrl: string;
  windowsUrl: string;
};

const productDownloads = (slug: string): ProductDownloads => ({
  macUrl: `${RELEASE_BASE}/${slug}.dmg`,
  windowsUrl: `${RELEASE_BASE}/${slug}.msi`,
});

export const alphaJX = productDownloads("Alpha.JX");
export const chorusR = productDownloads("Chorus-R");
export const fluffyverb = productDownloads("Fluffyverb");
export const poly81 = productDownloads("Poly.81");

export const downloadItems = (
  downloads: ProductDownloads,
): readonly { label: string; href: string }[] => [
  { label: "Installer (macOS)", href: downloads.macUrl },
  { label: "Installer (Windows)", href: downloads.windowsUrl },
];

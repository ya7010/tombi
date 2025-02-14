/**
 * Type definition for supported operating systems
 */
export type OperatingSystemType =
  | "mac"
  | "windows"
  | "linux"
  | "ios"
  | "android"
  | "unknown";

/**
 * Detects the operating system of the current runtime environment
 */
export function detectOperatingSystem(): OperatingSystemType {
  if (typeof window === "undefined") return "unknown";

  const ua = navigator.userAgent.toLowerCase();

  if (ua.includes("mac")) return "mac";
  if (ua.includes("win")) return "windows";
  if (ua.includes("linux")) return "linux";
  if (ua.includes("iphone") || ua.includes("ipad")) return "ios";
  if (ua.includes("android")) return "android";

  return "unknown";
}

/**
 * Checks if the current operating system is macOS
 * @deprecated Use detectOperatingSystem() instead
 */
export function isMacOS(): boolean {
  return detectOperatingSystem() === "mac";
}

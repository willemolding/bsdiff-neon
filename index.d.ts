
/**
 * Calculates a binary diff between two ArrayBuffers using the bsdiff algorithm
 * 
 * @param oldData The starting point for calculating the diff
 * @param newData The end point for calculating the diff
 */
export function diff(oldData: ArrayBuffer, newData: ArrayBuffer): ArrayBuffer;

/**
 * 
 * @param oldData The original starting point
 * @param diff The diff calculated by the diff function
 * @param size The size of the output. Note this should be included along with the transmission of the diff
 */
export function patch(oldData: ArrayBuffer, diff: ArrayBuffer, size: Number): number;

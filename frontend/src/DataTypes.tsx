export interface Bucket {
  uuid: string;
  bucket_name: string;
  bucket_slug: string;
  visible: boolean;
  drawing_enabled: boolean;
}

/**
 * Response for errors
 */
export interface ErrorResponse {
    message: string,
    canonical_reason: string
    error_code: number
}
export type Uuid = string;

export interface Bucket {
  uuid: string;
  bucket_name: string;
  bucket_slug: string;
  visible: boolean;
  drawing_enabled: boolean;
  private: boolean
}

export interface User {
  uuid: Uuid,
  google_name: string | null
}

/**
 * Response for errors
 */
export interface ErrorResponse {
  message: string,
  canonical_reason: string
  error_code: number
}


export interface Question {
  uuid: Uuid,
  bucket_uuid: Uuid,
  user_uuid: Uuid | null,
  question_text: String,
  archived: boolean
}

export interface ArchiveQuestionRequest {
  question_uuid: Uuid,
  archived: boolean
}

export interface NewQuestionRequest {
  bucket_uuid: Uuid,
  question_text: String
}

export interface BucketUserPermissions {
  set_visibility_permission: boolean,
  set_drawing_permission: boolean,
  set_private_permission: boolean,
  grant_permissions_permission: boolean
}


export interface BucketUserJoin {
  user_uuid: Uuid,
  bucket_uuid: Uuid,
  set_visibility_permission: boolean,
  set_drawing_permission: boolean,
  set_private_permission: boolean,
  grant_permissions_permission: boolean
}

// export interface BucketUserPermissionsChangeset {
//   set_visibility_permission?: boolean,
//   set_drawing_permission?: boolean ,
//   grant_permissions_permission?: boolean
// }

// TODO consider the ?: syntax instead of boolean | null
export interface SetPermissionsRequest {
    target_user_uuid: Uuid,
    set_visibility_permission: boolean| null,
    set_drawing_permission: boolean | null,
    set_private_permission: boolean | null,
    grant_permissions_permission: boolean | null
}

export interface LinkResponse {
  link: string
}

//! Bucket conversions

use crate::bucket::db_types::{Bucket, BucketFlagChangeset, NewBucket};
use wire;

impl Into<wire::bucket::Bucket> for Bucket {
    fn into(self) -> wire::bucket::Bucket {
        wire::bucket::Bucket {
            uuid: self.uuid,
            bucket_name: self.bucket_name,
            bucket_slug: self.bucket_slug,
            public_viewable: self.public_viewable,
            drawing_enabled: self.drawing_enabled,
            exclusive: self.exclusive,
            updated_at: self.updated_at,
            created_at: self.created_at,
        }
    }
}

impl From<wire::bucket::Bucket> for Bucket {
    fn from(bucket: wire::bucket::Bucket) -> Self {
        Bucket {
            uuid: bucket.uuid,
            bucket_name: bucket.bucket_name,
            bucket_slug: bucket.bucket_slug,
            public_viewable: bucket.public_viewable,
            drawing_enabled: bucket.drawing_enabled,
            exclusive: bucket.exclusive,
            updated_at: bucket.updated_at,
            created_at: bucket.created_at,
        }
    }
}

impl Into<wire::bucket::NewBucket> for NewBucket {
    fn into(self) -> wire::bucket::NewBucket {
        wire::bucket::NewBucket {
            bucket_name: self.bucket_name,
            bucket_slug: self.bucket_slug,
        }
    }
}

impl From<wire::bucket::NewBucket> for NewBucket {
    fn from(new_bucket: wire::bucket::NewBucket) -> Self {
        NewBucket {
            bucket_name: new_bucket.bucket_name,
            bucket_slug: new_bucket.bucket_slug,
        }
    }
}

impl Into<wire::bucket::BucketFlagChangeset> for BucketFlagChangeset {
    fn into(self) -> wire::bucket::BucketFlagChangeset {
        wire::bucket::BucketFlagChangeset {
            uuid: self.uuid,
            public_viewable: self.public_viewable,
            drawing_enabled: self.drawing_enabled,
            exclusive: self.exclusive,
        }
    }
}

impl From<wire::bucket::BucketFlagChangeset> for BucketFlagChangeset {
    fn from(bfc: wire::bucket::BucketFlagChangeset) -> Self {
        BucketFlagChangeset {
            uuid: bfc.uuid,
            public_viewable: bfc.public_viewable,
            drawing_enabled: bfc.drawing_enabled,
            exclusive: bfc.exclusive,
        }
    }
}

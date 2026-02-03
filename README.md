# Personal Cloud Photo & File Library

This project is a **personal cloud-native photo and file library** optimized for object storage.

## Docs

To deploy locally, will fetch most recent changes from main:
`./deploy local --fetch`

To deploy local with current workdir:
`./deploy local`

To deploy to server:
`./deploy remote`

You can also deploy directly with docker compose:
`ORIGIN=<origin> docker compose up --build`

Its important to set the origin to the origin url you plan to access the app from. The deploy script automatically sets it to localhost:3001 for local deploy and the server url origin for remote deploy.

Finally, each app can be manually started:
Api: `cargo run --bin api`
Frontend: `npm run dev`
The database should still be started with docker.

## Key Concepts

- **Immutable Blobs:**  
  All uploaded files (images or documents) are stored as **immutable objects** in object storage (e.g., Backblaze B2, S3). Once uploaded, data is never modified in place.

- **Local Metadata Database:**  
  A small database on the server tracks:
  - Object hashes (for deduplication)  
  - Object storage keys / locations  
  - Metadata (EXIF, MIME type, size, etc.)  
  - Logical organization: albums, folders, tags  

- **Logical Organization Only:**  
  Albums and folders are **views**, not physical directories. Moving a file or changing its album/folder only updates metadata — **no actual object data moves**.

- **Upload & Deduplication Workflow:**  
  1. File is uploaded to a temporary local storage/cache.  
  2. Server calculates hash and checks the database.  
  3. If new, the file is uploaded as a blob to object storage.  
  4. Database is updated with hash, storage key, and metadata.  
  5. Temporary file is removed.  

- **Read & Access:**  
  Users query the database to list files, albums, or folders. Blobs are fetched from object storage on demand.

- **Editing Files:**  
  Objects are immutable. To “edit” a file, download it, make changes locally, and re-upload as a new object.

- **Race Conditions:**  
  With a single-server setup, race conditions are minimal and mainly limited to metadata updates, which can be handled transactionally.

- **General Notes:**  
  - Supports images (JPEG, PNG, etc.) and generic files.  
  - Object storage versioning or lifecycle rules recommended for safety.  
  - Optional caching and thumbnail generation can be implemented locally for performance.

**Essentially:** the system separates **data (blobs)** from **organization & metadata (database)**, allowing unlimited cloud storage without local storage bloat, while still giving the user fast, flexible access and deduplication.
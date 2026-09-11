/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_INSPECTOR_RELEASE?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}

import js from '@eslint/js'
import tseslint from 'typescript-eslint'

// Flat-config equivalent of the old .eslintrc and .eslintignore:
// eslint:recommended plus typescript-eslint's recommended set.
export default tseslint.config(
  {
    // Generated output, never lint it.
    ignores: ['coverage/', 'dist/', 'node_modules/', 'src-tauri/target/', 'src-tauri/gen/'],
  },
  js.configs.recommended,
  tseslint.configs.recommended,
)

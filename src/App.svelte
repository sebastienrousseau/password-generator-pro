<script lang="ts">
  import { writable, get } from 'svelte/store'

  import './app.css'
  import CopyIcon from './Icon/CopyIcon.svelte'
  import GenerateIcon from './Icon/GenerateIcon.svelte'
  import ResetIcon from './Icon/ResetIcon.svelte'

  import { invoke } from '@tauri-apps/api/tauri'

  import Translate from './Components/I18n/i18n'

  const systemLanguage = Intl.DateTimeFormat().resolvedOptions().locale
  // const systemLanguage = navigator.languages[0]
  // const systemLanguage = 'pl-PL'

  const generate_i18n = Translate('Button', systemLanguage)
  const label_i18n = Translate('Label', systemLanguage)
  const label2_i18n = Translate('Label2', systemLanguage)
  const label3_i18n = Translate('Label3', systemLanguage)
  const label4_i18n = Translate('Label4', systemLanguage)
  const placeholder_i18n = Translate('Placeholder', systemLanguage)
  const placeholder2_i18n = Translate('Placeholder2', systemLanguage)
  const reset_i18n = Translate('Button2', systemLanguage)
  const subtitle_i18n = Translate('Subtitle', systemLanguage)
  const title_i18n = Translate('Title', systemLanguage)

  const defaultFormValues = {
    len: 3,
    separator: '-',
  }

  let password: string | undefined
  let hash: string | undefined

  const formData = writable({
    len: defaultFormValues.len,
    separator: defaultFormValues.separator,
  })

  function onReset() {
    formData.set(defaultFormValues)
    password = undefined
    hash = undefined
  }

  async function onGenerate() {
    // Log that it was generated...
    console.log('Calling on-generate...')

    // Extract the form data
    const { len, separator } = get(formData)
    console.log('len: ', len, 'separator: ', separator)

    try {
      // Call the backend generator
      const data: { password: string; hash: string } = await invoke('generate_password', {
        len,
        separator,
      })

      console.log('Returned successfully: ', data)

      // Set the password value
      password = data.password
      hash = data.hash
    } catch (err) {
      // Log any error that occurs
      console.error('Failed to generate password:', err)
    }
  }

  function onCopy(data: string) {
    navigator.clipboard.writeText(data)
  }

  function onCopyPassword() {
    onCopy(password)
    document.querySelector('#generated-password')
  }

  function onCopyhash() {
    onCopy(hash)
  }
</script>

<main class="absolute w-full h-full bg-slate-800 text-slate-50 uppercase">
  <!-- Page Header -->
  <div class="my-5 mx-6 font-mono">
    <h1 class="text-3xl text-center font-bold mb-1">
      {title_i18n}
    </h1>
    <p class="text-md text-center mb-7 text-slate-500">
      {subtitle_i18n}
    </p>

    <!-- Password Display -->
    <p class="text-md mb-2">{label_i18n}</p>
    <div class="flex bg-slate-700 rounded-xl w-96 mb-5 overflow-hidden">
      <span class="px-3 py-2 flex-grow select-all normal-case truncate">
        {#if password}
          <span id="generated-password">
            {password}
          </span>
        {:else}
          <span class="text-slate-400 select-none">{placeholder_i18n}</span>
        {/if}
      </span>
      <button
        on:click={onCopyPassword}
        class="py-2 px-1 bg-sky-500 hover:bg-sky-400 active:bg-sky-600"
      >
        <CopyIcon />
      </button>
    </div>

    <p class="text-md mb-2">{label2_i18n}</p>
    <div class="flex bg-slate-700 rounded-xl w-96 mb-5 overflow-hidden">
      <span class="px-3 py-2 flex-grow select-all normal-case truncate">
        {#if hash}
          <span id="generated-hash">
            {hash}
          </span>
        {:else}
          <span class="text-slate-400 select-none">{placeholder2_i18n}</span>
        {/if}
      </span>
      <button on:click={onCopyhash} class="py-2 px-1 bg-sky-500 hover:bg-sky-400 active:bg-sky-600">
        <CopyIcon />
      </button>
    </div>

    <br />

    <!-- Page Header -->
    <form class="text-sm">
      <div class="block mb-4">
        <label for="num-words" class="mb-2">{label3_i18n}</label>
        <input
          type="number"
          name="num-words"
          min="3"
          max="9"
          bind:value={$formData.len}
          class="border-gray-300 p-2 rounded-lg block mt-1 bg-slate-700 w-12"
        />
      </div>
      <div class="block mb-4">
        <label for="num-words" class="mb-2">{label4_i18n}</label>
        <input
          type="text"
          name="separator"
          maxlength="1"
          bind:value={$formData.separator}
          class="border-gray-300 p-2 rounded-lg block mt-1 bg-slate-700 w-12"
        />
      </div>

      <!-- Action Buttons -->
      <div class="mt-7 flex space-x-3 justify-center">
        <button
          type="button"
          class="flex items-center p-2 bg-slate-500 hover:bg-slate-400 active:bg-slate-600 rounded-xl"
          on:click={onReset}
        >
          <span class="mr-2 align-bottom">
            <ResetIcon />
          </span>
          <span class="uppercase">{reset_i18n}</span>
        </button>

        <button
          type="button"
          class="flex items-center p-2 bg-sky-500 hover:bg-sky-400 active:bg-sky-600 rounded-xl"
          on:click={async () => {
            await onGenerate()
          }}
        >
          <span class="mr-2 align-bottom">
            <GenerateIcon />
          </span>
          <span class="uppercase">{generate_i18n}</span>
        </button>
      </div>
    </form>
  </div>
</main>

<style>
</style>

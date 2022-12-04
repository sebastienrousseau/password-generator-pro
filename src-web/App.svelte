<script lang="ts">
  import { writable, get } from 'svelte/store'

  import './app.css'
  import CopyIcon from './Icon/CopyIcon.svelte'
  import GenerateIcon from './Icon/GenerateIcon.svelte'
  import ResetIcon from './Icon/ResetIcon.svelte'

  import { invoke } from '@tauri-apps/api/tauri'

  import Translate from './Components/i18n'

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

  async function onReset() {
    await invoke('play_audio', { sound: 'reset.ogg' }).then(() => {
      formData.set(defaultFormValues)
      password = undefined
      hash = undefined
    })
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

      await sleep(250).then(() => {
        invoke('play_audio', { sound: 'generate.ogg' })
      })
    } catch (err) {
      // Log any error that occurs
      console.error('Failed to generate password:', err)
    }
  }

  function sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms ?? 200))
  }

  async function onCopy(data: string) {
    await invoke('play_audio', { sound: 'copy.ogg' }).then(() => {
      navigator.clipboard.writeText(data)
    })
  }

  function onCopyPassword() {
    onCopy(password)
    document.querySelector('#generated-password')
  }

  function onCopyhash() {
    onCopy(hash)
  }

  const systemLanguage = Intl.DateTimeFormat().resolvedOptions().locale
  // const systemLanguage = navigator.languages[0]
  // const systemLanguage = 'en-GB'

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
</script>

<main
  class="absolute antialiased sm:subpixel-antialiased md:antialiased lg:antialiased bg-white dark:bg-[#342927] h-full font-mono justify-center min-w-340 text-lg text-white w-full"
>
  <!-- Page Header -->
  <div class="my-5 mx-5">
    <h1 class="text-3xl text-center font-bold mb-1 text-black dark:text-white uppercase">
      {title_i18n}
    </h1>
    <p class="md:text-lg text-center capitalize mb-7 text-black dark:text-white">
      {subtitle_i18n}
    </p>

    <!-- Password Display -->
    <p class="text-black capitalize dark:text-white mb-2">{label_i18n}</p>
    <div
      class="flex bg-white border border-[#dedede] dark:border-[#736865] dark:bg-[#403533] rounded-xl w-full mb-5 overflow-hidden"
    >
      <span class="px-3 py-2 flex-grow select-all normal-case truncate">
        {#if password}
          <span id="generated-password" class="text-black dark:text-white">
            {password}
          </span>
        {:else}
          <span class="text-[#b3aead] dark:text-[#706765] select-none">{placeholder_i18n}</span>
        {/if}
      </span>
      <button
        on:click={onCopyPassword}
        class="py-2 px-1 bg-blue-light dark:bg-blue-dark hover:bg-blue-600 active:bg-blue-700"
      >
        <CopyIcon />
      </button>
    </div>

    <p class="text-black capitalize dark:text-white mb-2">{label2_i18n}</p>
    <div
      class="flex border border-[#dedede] dark:border-[#736865] bg-white dark:bg-[#403533] rounded-xl w-full mb-5 overflow-hidden"
    >
      <span class="px-3 py-2 flex-grow select-all normal-case truncate">
        {#if hash}
          <span id="generated-hash" class="text-black dark:text-white">
            {hash}
          </span>
        {:else}
          <span class="text-[#b3aead] dark:text-[#706765] select-none">{placeholder2_i18n}</span>
        {/if}
      </span>
      <button
        on:click={onCopyhash}
        class="text-black dark:text-white py-2 px-1 bg-blue-light dark:bg-blue-dark hover:bg-blue-600 active:bg-blue-700"
      >
        <CopyIcon />
      </button>
    </div>

    <!-- Page Header -->
    <form>
      <div class="grid grid-cols-2 gap-4">
        <div class="block mb-4">
          <label for="num-words" class="text-black dark:text-white capitalize mb-2"
            >{label3_i18n}</label
          >
          <input
            type="number"
            name="num-words"
            min="3"
            max="9"
            bind:value={$formData.len}
            class="border border-[#dedede] dark:border-[#736865] text-black dark:text-white text-right s-2 border-gray-light dark:border-gray-dark p-2 rounded-lg block mt-1 bg-white dark:bg-[#403533] w-full"
          />
        </div>
        <div class="block mb-4">
          <label for="num-words" class="text-black dark:text-white capitalize mb-2"
            >{label4_i18n}</label
          >
          <input
            type="text"
            name="separator"
            maxlength="1"
            bind:value={$formData.separator}
            class="border border-[#dedede] dark:border-[#736865] text-black dark:text-white text-right s-2 border-gray-light dark:border-gray-dark p-2 rounded-lg block mt-1 bg-white dark:bg-[#403533] w-full"
          />
        </div>
      </div>
      <!-- Action Buttons -->
      <div class="mt-7 flex space-x-3 justify-center">
        <button
          type="button"
          class="min-w-fit flex justify-center p-2 bg-gray-light dark:bg-gray-dark hover:bg-gray-600 active:bg-gray-700 rounded-lg px-6 shadow-md shadow-gray-400 dark:shadow-none"
          on:click={onReset}
        >
          <span class="mr-2 align-bottom">
            <ResetIcon />
          </span>
          <span> {reset_i18n} </span>
        </button>

        <button
          type="button"
          class="min-w-fit flex justify-center p-2 bg-blue-light dark:bg-blue-dark hover:bg-blue-600 active:bg-blue-700 rounded-lg px-6 shadow-md shadow-gray-400 dark:shadow-none"
          on:click={async () => {
            await onGenerate()
          }}
        >
          <span class="mr-2 align-bottom">
            <GenerateIcon />
          </span>
          <span> {generate_i18n} </span>
        </button>
      </div>
    </form>
  </div>
</main>

<style>
</style>

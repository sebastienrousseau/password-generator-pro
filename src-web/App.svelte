<script lang="ts">
  import { get, writable } from 'svelte/store'
  import './app.css'
  import CopyIcon from './Icon/CopyIcon.svelte'
  import GenerateIcon from './Icon/GenerateIcon.svelte'
  import ResetIcon from './Icon/ResetIcon.svelte'

  import { writeText } from '@tauri-apps/api/clipboard'
  import { invoke } from '@tauri-apps/api/tauri'

  import Translate from './Components/i18n'

  const defaultFormValues = {
    len: 3,
    separator: '-',
  }

  let password: string | undefined
  let qrcode: string | undefined
  let hash: string | undefined
  let uuid: string | undefined

  const formData = writable({
    len: defaultFormValues.len,
    separator: defaultFormValues.separator,
  })

  // This function is used to play the range sound.
  async function onRangeChange() {
    // Play the range sound
    const range = new Audio('./sounds/range.mp3')
    range.play()
  }

  // This function is used to reset the form data.
  async function onReset() {
    // Play the reset sound
    const reset = new Audio('./sounds/whoosh.mp3')
    reset.play()

    // Wait for the reset to finish
    await sleep(250)

    // Reset the form data
    formData.set(defaultFormValues)
    password = undefined
    hash = undefined
    qrcode = undefined
    uuid = undefined
  }

  // This function is used to generate a password.
  async function onGenerate() {
    // Log that it was generated...
    console.log('Calling on-generate...')

    // Extract the form data
    const { len, separator } = get(formData)
    console.log('len: ', len, 'separator: ', separator)

    try {
      // Call the backend generator
      const data: { password: string; hash: string; qrcode: string; uuid: string } = await invoke(
        'generate_password',
        {
          len,
          separator,
        }
      )

      console.log('Returned successfully: ', data)

      // Play the generate sound
      const generate = new Audio('./sounds/generate.mp3')
      generate.play()

      // Wait for the generator to finish
      await sleep(250)

      // Set the password value
      password = data.password
      hash = data.hash
      uuid = data.uuid
      qrcode = data.qrcode
    } catch (err) {
      // Log any error that occurs
      console.error('Failed to generate password:', err)
    }
  }

  // This function is used to sleep for a given amount of time.
  function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms ?? 200))
  }

  // This function is used to copy the given data to the clipboard.
  async function onCopy(data: string) {
    // Play the copy sound
    const copy = new Audio('./sounds/copy.mp3')
    copy.play()

    // Wait for the copy to finish
    await sleep(250)

    await writeText(data)

    let result: string = ''
    const handleClick = async () => {
      result = await invoke('logger', {
        time: 123,
        info: 'Info',
        message: 'SystemTrayEvent',
        details: data,
      })
    }
    // navigator.clipboard.writeText(data)
  }

  // This function is used to copy the password to the clipboard.
  function onCopyPassword() {
    onCopy(String(password))
    document.querySelector('#generated-password')
  }

  // This function is used to copy the hash to the clipboard.
  function onCopyhash() {
    onCopy(String(hash))
  }

  // This function is used to copy the uuid to the clipboard.
  function onCopyuuid() {
    onCopy(String(uuid))
  }

  const systemLanguage = Intl.DateTimeFormat().resolvedOptions().locale
  const generate_i18n = Translate('Button', systemLanguage)
  const label_i18n = Translate('Label', systemLanguage)
  const label2_i18n = Translate('Label2', systemLanguage)
  const label3_i18n = Translate('Label3', systemLanguage)
  const label4_i18n = Translate('Label4', systemLanguage)
  const label5_i18n = Translate('Label5', systemLanguage)
  const placeholder_i18n = Translate('Placeholder', systemLanguage)
  const placeholder2_i18n = Translate('Placeholder2', systemLanguage)
  const placeholder3_i18n = Translate('Placeholder3', systemLanguage)
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
        on:click={async () => {
          await onCopyPassword()
        }}
        class="py-2 px-1 bg-blue-light dark:bg-blue-dark hover:bg-blue-600 active:bg-blue-700"
      >
        <CopyIcon />
      </button>
    </div>

    <div class="grid grid-cols-5 gap-3">
      <p class="text-black capitalize dark:text-white mb-2 w-fit">{label2_i18n}</p>
      <div
        class="col-span-4 flex border border-[#dedede] dark:border-[#736865] bg-white dark:bg-[#403533] rounded-xl w-full mb-5 overflow-hidden"
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
          on:click={async () => {
            await onCopyhash()
          }}
          class="text-black dark:text-white py-2 px-1 bg-blue-light dark:bg-blue-dark hover:bg-blue-600 active:bg-blue-700"
        >
          <CopyIcon />
        </button>
      </div>
    </div>
    <div class="grid grid-cols-5 gap-3">
      <p class="text-black capitalize dark:text-white mb-2 w-fit">{label5_i18n}</p>
      <div
        class="col-span-4 flex border border-[#dedede] dark:border-[#736865] bg-white dark:bg-[#403533] rounded-xl w-full mb-5 overflow-hidden"
      >
        <span class="px-3 py-2 flex-grow select-all normal-case truncate">
          {#if uuid}
            <span id="generated-hash" class="text-black dark:text-white">
              {uuid}
            </span>
          {:else}
            <span class="text-[#b3aead] dark:text-[#706765] select-none">{placeholder3_i18n}</span>
          {/if}
        </span>
        <button
          on:click={async () => {
            await onCopyuuid()
          }}
          class="text-black dark:text-white py-2 px-1 bg-blue-light dark:bg-blue-dark hover:bg-blue-600 active:bg-blue-700"
        >
          <CopyIcon />
        </button>
      </div>
    </div>

    <!-- Page Header -->
    <form>
      <div class="grid grid-cols-5 gap-3">
        <div class="col-span-4 block mb-4">
          <label for="numWords" class="text-black dark:text-white capitalize mb-2"
            >{label3_i18n}</label
          >
          <input
            type="range"
            name="numWords"
            class="align-bottom w-full h-4 p-0 bg-blue-light dark:bg-blue-dark hover:bg-blue-600 active:bg-blue-700 rounded-lg appearance-none cursor-pointer focus:outline-none focus:ring-0 focus:shadow-none"
            min="3"
            max="9"
            step="1"
            bind:value={$formData.len}
            on:change={async () => {
              await onRangeChange()
            }}
            id="numWords"
          />
          <div class="align-bottom flex w-full justify-between">
            <span class="p-1 text-black dark:text-white">3</span>
            <span class="p-1 text-black dark:text-white">4</span>
            <span class="p-1 text-black dark:text-white">5</span>
            <span class="p-1 text-black dark:text-white">6</span>
            <span class="p-1 text-black dark:text-white">7</span>
            <span class="p-1 text-black dark:text-white">8</span>
            <span class="p-1 text-black dark:text-white">9</span>
          </div>
          <!-- <input
            type="number"
            name="numWords"
            min="3"
            max="9"
            bind:value={$formData.len}
            class="border border-[#dedede] dark:border-[#736865] text-black dark:text-white text-right s-2 border-gray-light dark:border-gray-dark p-2 rounded-lg block mt-1 bg-white dark:bg-[#403533] w-full"
          /> -->
        </div>
        <div class="block mb-4">
          <label for="large" class="text-black dark:text-white capitalize mb-2">{label4_i18n}</label
          >

          <select
            class="
            appearance-none
            active:bg-blue-700
            bg-blue-light dark:bg-blue-dark
            block
            cursor-pointer
            ease-in-out
            focus:outline-none
            focus:ring-0
            focus:shadow-none
            font-normal
            form-select
            form-select-lg
            hover:bg-blue-600
            px-4
            py-2
            rounded-lg
            text-xl
            text-center
            transition
            w-full
            "
            bind:value={$formData.separator}
            on:change={async () => {
              await onRangeChange()
            }}
          >
            <option class="" value="!">!</option>
            <option value="@">@</option>
            <option value="$">$</option>
            <option value="%">%</option>
            <option value="^">^</option>
            <option value="&">&</option>
            <option value="*">*</option>
            <option selected value="-">-</option>
            <option value="_">_</option>
            <option value="+">+</option>
            <option value="=">=</option>
            <option value=":">:</option>
            <option value="|">|</option>
            <option value="~">~</option>
            <option value="?">?</option>
            <option value="/">/</option>
            <option value=".">.</option>
            <option value=";">;</option>
          </select>
          <!-- <label for="numWords" class="text-black dark:text-white capitalize mb-2"
            >{label4_i18n}</label
          >
          <input
            type="text"
            name="separator"
            maxlength="1"
            bind:value={$formData.separator}
            class="border border-[#dedede] dark:border-[#736865] text-black dark:text-white text-right s-2 border-gray-light dark:border-gray-dark p-2 rounded-lg block mt-1 bg-white dark:bg-[#403533] w-full"
          /> -->
        </div>
      </div>
      <!-- Action Buttons -->
      <div class="mt-7 flex space-x-3 justify-center">
        <button
          type="button"
          class="min-w-fit flex justify-center p-2 bg-gray-light dark:bg-gray-dark hover:bg-gray-600 active:bg-gray-700 rounded-lg px-6 shadow-md shadow-gray-400 dark:shadow-none"
          on:click={async () => {
            await onReset()
            $formData.len = 3
          }}
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

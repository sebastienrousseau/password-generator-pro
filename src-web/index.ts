import './style.css'
import App from './App.svelte'

// `getElementById` is typed `HTMLElement | null`, and mounting onto
// null fails deep inside Svelte with no indication of the cause. A
// missing mount point is a broken build, so say that instead.
const target = document.getElementById('app')
if (!target) {
  throw new Error('mount point #app is missing from index.html')
}

const app = new App({ target })

export default app

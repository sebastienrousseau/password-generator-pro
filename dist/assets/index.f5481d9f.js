;(function () {
  const e = document.createElement('link').relList
  if (e && e.supports && e.supports('modulepreload')) return
  for (const o of document.querySelectorAll('link[rel="modulepreload"]')) r(o)
  new MutationObserver((o) => {
    for (const i of o)
      if (i.type === 'childList')
        for (const f of i.addedNodes) f.tagName === 'LINK' && f.rel === 'modulepreload' && r(f)
  }).observe(document, { childList: !0, subtree: !0 })
  function n(o) {
    const i = {}
    return (
      o.integrity && (i.integrity = o.integrity),
      o.referrerpolicy && (i.referrerPolicy = o.referrerpolicy),
      o.crossorigin === 'use-credentials'
        ? (i.credentials = 'include')
        : o.crossorigin === 'anonymous'
        ? (i.credentials = 'omit')
        : (i.credentials = 'same-origin'),
      i
    )
  }
  function r(o) {
    if (o.ep) return
    o.ep = !0
    const i = n(o)
    fetch(o.href, i)
  }
})()
function p() {}
function Te(t) {
  return t()
}
function Le() {
  return Object.create(null)
}
function Y(t) {
  t.forEach(Te)
}
function Ve(t) {
  return typeof t == 'function'
}
function Z(t, e) {
  return t != t ? e == e : t !== e || (t && typeof t == 'object') || typeof t == 'function'
}
function We(t) {
  return Object.keys(t).length === 0
}
function ze(t, ...e) {
  if (t == null) return p
  const n = t.subscribe(...e)
  return n.unsubscribe ? () => n.unsubscribe() : n
}
function Je(t) {
  let e
  return ze(t, (n) => (e = n))(), e
}
function Qe(t, e, n) {
  t.$$.on_destroy.push(ze(e, n))
}
function l(t, e) {
  t.appendChild(e)
}
function R(t, e, n) {
  t.insertBefore(e, n || null)
}
function P(t) {
  t.parentNode && t.parentNode.removeChild(t)
}
function c(t) {
  return document.createElement(t)
}
function T(t) {
  return document.createElementNS('http://www.w3.org/2000/svg', t)
}
function ye(t) {
  return document.createTextNode(t)
}
function m() {
  return ye(' ')
}
function F(t, e, n, r) {
  return t.addEventListener(e, n, r), () => t.removeEventListener(e, n, r)
}
function s(t, e, n) {
  n == null ? t.removeAttribute(e) : t.getAttribute(e) !== n && t.setAttribute(e, n)
}
function He(t) {
  return t === '' ? null : +t
}
function Xe(t) {
  return Array.from(t.childNodes)
}
function Ue(t, e) {
  ;(e = '' + e), t.wholeText !== e && (t.data = e)
}
function ae(t, e) {
  t.value = e == null ? '' : e
}
let $e
function X(t) {
  $e = t
}
const K = [],
  Be = [],
  fe = [],
  qe = [],
  Ye = Promise.resolve()
let be = !1
function Ze() {
  be || ((be = !0), Ye.then(De))
}
function _e(t) {
  fe.push(t)
}
const he = new Set()
let ce = 0
function De() {
  const t = $e
  do {
    for (; ce < K.length; ) {
      const e = K[ce]
      ce++, X(e), et(e.$$)
    }
    for (X(null), K.length = 0, ce = 0; Be.length; ) Be.pop()()
    for (let e = 0; e < fe.length; e += 1) {
      const n = fe[e]
      he.has(n) || (he.add(n), n())
    }
    fe.length = 0
  } while (K.length)
  for (; qe.length; ) qe.pop()()
  ;(be = !1), he.clear(), X(t)
}
function et(t) {
  if (t.fragment !== null) {
    t.update(), Y(t.before_update)
    const e = t.dirty
    ;(t.dirty = [-1]), t.fragment && t.fragment.p(t.ctx, e), t.after_update.forEach(_e)
  }
}
const de = new Set()
let tt
function W(t, e) {
  t && t.i && (de.delete(t), t.i(e))
}
function ie(t, e, n, r) {
  if (t && t.o) {
    if (de.has(t)) return
    de.add(t),
      tt.c.push(() => {
        de.delete(t), r && (n && t.d(1), r())
      }),
      t.o(e)
  } else r && r()
}
function ue(t) {
  t && t.c()
}
function J(t, e, n, r) {
  const { fragment: o, after_update: i } = t.$$
  o && o.m(e, n),
    r ||
      _e(() => {
        const f = t.$$.on_mount.map(Te).filter(Ve)
        t.$$.on_destroy ? t.$$.on_destroy.push(...f) : Y(f), (t.$$.on_mount = [])
      }),
    i.forEach(_e)
}
function Q(t, e) {
  const n = t.$$
  n.fragment !== null &&
    (Y(n.on_destroy),
    n.fragment && n.fragment.d(e),
    (n.on_destroy = n.fragment = null),
    (n.ctx = []))
}
function nt(t, e) {
  t.$$.dirty[0] === -1 && (K.push(t), Ze(), t.$$.dirty.fill(0)),
    (t.$$.dirty[(e / 31) | 0] |= 1 << e % 31)
}
function pe(t, e, n, r, o, i, f, d = [-1]) {
  const g = $e
  X(t)
  const a = (t.$$ = {
    fragment: null,
    ctx: [],
    props: i,
    update: p,
    not_equal: o,
    bound: Le(),
    on_mount: [],
    on_destroy: [],
    on_disconnect: [],
    before_update: [],
    after_update: [],
    context: new Map(e.context || (g ? g.$$.context : [])),
    callbacks: Le(),
    dirty: d,
    skip_bound: !1,
    root: e.target || g.$$.root,
  })
  f && f(a.root)
  let y = !1
  if (
    ((a.ctx = n
      ? n(t, e.props || {}, (h, $, ..._) => {
          const O = _.length ? _[0] : $
          return (
            a.ctx &&
              o(a.ctx[h], (a.ctx[h] = O)) &&
              (!a.skip_bound && a.bound[h] && a.bound[h](O), y && nt(t, h)),
            $
          )
        })
      : []),
    a.update(),
    (y = !0),
    Y(a.before_update),
    (a.fragment = r ? r(a.ctx) : !1),
    e.target)
  ) {
    if (e.hydrate) {
      const h = Xe(e.target)
      a.fragment && a.fragment.l(h), h.forEach(P)
    } else a.fragment && a.fragment.c()
    e.intro && W(t.$$.fragment), J(t, e.target, e.anchor, e.customElement), De()
  }
  X(g)
}
class me {
  $destroy() {
    Q(this, 1), (this.$destroy = p)
  }
  $on(e, n) {
    if (!Ve(n)) return p
    const r = this.$$.callbacks[e] || (this.$$.callbacks[e] = [])
    return (
      r.push(n),
      () => {
        const o = r.indexOf(n)
        o !== -1 && r.splice(o, 1)
      }
    )
  }
  $set(e) {
    this.$$set && !We(e) && ((this.$$.skip_bound = !0), this.$$set(e), (this.$$.skip_bound = !1))
  }
}
const G = []
function rt(t, e = p) {
  let n
  const r = new Set()
  function o(d) {
    if (Z(t, d) && ((t = d), n)) {
      const g = !G.length
      for (const a of r) a[1](), G.push(a, t)
      if (g) {
        for (let a = 0; a < G.length; a += 2) G[a][0](G[a + 1])
        G.length = 0
      }
    }
  }
  function i(d) {
    o(d(t))
  }
  function f(d, g = p) {
    const a = [d, g]
    return (
      r.add(a),
      r.size === 1 && (n = e(o) || p),
      d(t),
      () => {
        r.delete(a), r.size === 0 && (n(), (n = null))
      }
    )
  }
  return { set: o, update: i, subscribe: f }
}
function st(t) {
  let e, n
  return {
    c() {
      ;(e = T('svg')),
        (n = T('path')),
        s(n, 'stroke-linecap', 'round'),
        s(n, 'stroke-linejoin', 'round'),
        s(
          n,
          'd',
          'M15.666 3.888A2.25 2.25 0 0013.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 01-.75.75H9a.75.75 0 01-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 01-2.25 2.25H6.75A2.25 2.25 0 014.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 011.927-.184'
        ),
        s(
          e,
          'class',
          'h-7 aspect-square mx-1 stroke-5 stroke-slate-50 hover:stroke-slate-100 active:stroke-slate-50'
        ),
        s(e, 'xmlns', 'http://www.w3.org/2000/svg'),
        s(e, 'fill', 'none'),
        s(e, 'viewBox', '0 0 24 24'),
        s(e, 'stroke-width', '1.5'),
        s(e, 'stroke', 'currentColor')
    },
    m(r, o) {
      R(r, e, o), l(e, n)
    },
    p,
    i: p,
    o: p,
    d(r) {
      r && P(e)
    },
  }
}
class Fe extends me {
  constructor(e) {
    super(), pe(this, e, null, st, Z, {})
  }
}
function ot(t) {
  let e, n
  return {
    c() {
      ;(e = T('svg')),
        (n = T('path')),
        s(n, 'stroke-linecap', 'round'),
        s(n, 'stroke-linejoin', 'round'),
        s(
          n,
          'd',
          'M19.5 12c0-1.232-.046-2.453-.138-3.662a4.006 4.006 0 00-3.7-3.7 48.678 48.678 0 00-7.324 0 4.006 4.006 0 00-3.7 3.7c-.017.22-.032.441-.046.662M19.5 12l3-3m-3 3l-3-3m-12 3c0 1.232.046 2.453.138 3.662a4.006 4.006 0 003.7 3.7 48.656 48.656 0 007.324 0 4.006 4.006 0 003.7-3.7c.017-.22.032-.441.046-.662M4.5 12l3 3m-3-3l-3 3'
        ),
        s(e, 'class', 'w-6 h-6'),
        s(e, 'xmlns', 'http://www.w3.org/2000/svg'),
        s(e, 'fill', 'none'),
        s(e, 'viewBox', '0 0 24 24'),
        s(e, 'stroke-width', '1.5'),
        s(e, 'stroke', 'currentColor')
    },
    m(r, o) {
      R(r, e, o), l(e, n)
    },
    p,
    i: p,
    o: p,
    d(r) {
      r && P(e)
    },
  }
}
class lt extends me {
  constructor(e) {
    super(), pe(this, e, null, ot, Z, {})
  }
}
function at(t) {
  let e, n
  return {
    c() {
      ;(e = T('svg')),
        (n = T('path')),
        s(n, 'stroke-linecap', 'round'),
        s(n, 'stroke-linejoin', 'round'),
        s(
          n,
          'd',
          'M15.59 14.37a6 6 0 01-5.84 7.38v-4.8m5.84-2.58a14.98 14.98 0 006.16-12.12A14.98 14.98 0 009.631 8.41m5.96 5.96a14.926 14.926 0 01-5.841 2.58m-.119-8.54a6 6 0 00-7.381 5.84h4.8m2.581-5.84a14.927 14.927 0 00-2.58 5.84m2.699 2.7c-.103.021-.207.041-.311.06a15.09 15.09 0 01-2.448-2.448 14.9 14.9 0 01.06-.312m-2.24 2.39a4.493 4.493 0 00-1.757 4.306 4.493 4.493 0 004.306-1.758M16.5 9a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0z'
        ),
        s(e, 'class', 'w-6 h-6'),
        s(e, 'xmlns', 'http://www.w3.org/2000/svg'),
        s(e, 'fill', 'none'),
        s(e, 'viewBox', '0 0 24 24'),
        s(e, 'stroke-width', '1.5'),
        s(e, 'stroke', 'currentColor')
    },
    m(r, o) {
      R(r, e, o), l(e, n)
    },
    p,
    i: p,
    o: p,
    d(r) {
      r && P(e)
    },
  }
}
class ct extends me {
  constructor(e) {
    super(), pe(this, e, null, at, Z, {})
  }
}
var it = Object.defineProperty,
  ut = (t, e) => {
    for (var n in e) it(t, n, { get: e[n], enumerable: !0 })
  },
  ft = {}
ut(ft, { convertFileSrc: () => pt, invoke: () => Ke, transformCallback: () => we })
function dt() {
  return window.crypto.getRandomValues(new Uint32Array(1))[0]
}
function we(t, e = !1) {
  let n = dt(),
    r = `_${n}`
  return (
    Object.defineProperty(window, r, {
      value: (o) => (e && Reflect.deleteProperty(window, r), t == null ? void 0 : t(o)),
      writable: !1,
      configurable: !0,
    }),
    n
  )
}
async function Ke(t, e = {}) {
  return new Promise((n, r) => {
    let o = we((f) => {
        n(f), Reflect.deleteProperty(window, `_${i}`)
      }, !0),
      i = we((f) => {
        r(f), Reflect.deleteProperty(window, `_${o}`)
      }, !0)
    window.__TAURI_IPC__({ cmd: t, callback: o, error: i, ...e })
  })
}
function pt(t, e = 'asset') {
  let n = encodeURIComponent(t)
  return navigator.userAgent.includes('Windows')
    ? `https://${e}.localhost/${n}`
    : `${e}://localhost/${n}`
}
function mt(t) {
  let e
  return {
    c() {
      ;(e = c('span')),
        (e.textContent = 'No password generated yet'),
        s(e, 'class', 'text-slate-400 select-none')
    },
    m(n, r) {
      R(n, e, r)
    },
    p,
    d(n) {
      n && P(e)
    },
  }
}
function gt(t) {
  let e, n
  return {
    c() {
      ;(e = c('span')), (n = ye(t[0])), s(e, 'id', 'generated-password')
    },
    m(r, o) {
      R(r, e, o), l(e, n)
    },
    p(r, o) {
      o & 1 && Ue(n, r[0])
    },
    d(r) {
      r && P(e)
    },
  }
}
function ht(t) {
  let e
  return {
    c() {
      ;(e = c('span')),
        (e.textContent = 'No hash generated yet'),
        s(e, 'class', 'text-slate-400 select-none')
    },
    m(n, r) {
      R(n, e, r)
    },
    p,
    d(n) {
      n && P(e)
    },
  }
}
function bt(t) {
  let e, n
  return {
    c() {
      ;(e = c('span')), (n = ye(t[1])), s(e, 'id', 'generated-hash')
    },
    m(r, o) {
      R(r, e, o), l(e, n)
    },
    p(r, o) {
      o & 2 && Ue(n, r[1])
    },
    d(r) {
      r && P(e)
    },
  }
}
function _t(t) {
  let e,
    n,
    r,
    o,
    i,
    f,
    d,
    g,
    a,
    y,
    h,
    $,
    _,
    O,
    C,
    V,
    b,
    z,
    ve,
    H,
    S,
    xe,
    ke,
    Ce,
    A,
    I,
    U,
    Pe,
    w,
    Ae,
    M,
    D,
    Ee,
    v,
    Ne,
    L,
    E,
    ee,
    B,
    je,
    te,
    Re,
    N,
    ne,
    q,
    Oe,
    re,
    se,
    ge,
    Se
  function Ie(u, j) {
    return u[0] ? gt : mt
  }
  let oe = Ie(t),
    x = oe(t)
  _ = new Fe({})
  function Me(u, j) {
    return u[1] ? bt : ht
  }
  let le = Me(t),
    k = le(t)
  return (
    (S = new Fe({})),
    (B = new lt({})),
    (q = new ct({})),
    {
      c() {
        ;(e = c('main')),
          (n = c('div')),
          (r = c('h1')),
          (r.textContent = 'Password Generator Pro'),
          (o = m()),
          (i = c('p')),
          (i.textContent = 'A fast, simple and powerful password generator.'),
          (f = m()),
          (d = c('p')),
          (d.textContent = 'Password'),
          (g = m()),
          (a = c('div')),
          (y = c('span')),
          x.c(),
          (h = m()),
          ($ = c('button')),
          ue(_.$$.fragment),
          (O = m()),
          (C = c('p')),
          (C.textContent = 'Bcrypt Hash'),
          (V = m()),
          (b = c('div')),
          (z = c('span')),
          k.c(),
          (ve = m()),
          (H = c('button')),
          ue(S.$$.fragment),
          (xe = m()),
          (ke = c('br')),
          (Ce = m()),
          (A = c('form')),
          (I = c('div')),
          (U = c('label')),
          (U.textContent = 'Number of words'),
          (Pe = m()),
          (w = c('input')),
          (Ae = m()),
          (M = c('div')),
          (D = c('label')),
          (D.textContent = 'Separator'),
          (Ee = m()),
          (v = c('input')),
          (Ne = m()),
          (L = c('div')),
          (E = c('button')),
          (ee = c('span')),
          ue(B.$$.fragment),
          (je = m()),
          (te = c('span')),
          (te.textContent = 'Reset'),
          (Re = m()),
          (N = c('button')),
          (ne = c('span')),
          ue(q.$$.fragment),
          (Oe = m()),
          (re = c('span')),
          (re.textContent = 'Generate'),
          s(r, 'class', 'text-3xl text-center font-bold mb-1'),
          s(i, 'class', 'text-md text-center mb-7 text-slate-500'),
          s(d, 'class', 'text-md mb-2'),
          s(y, 'class', 'px-3 py-2 flex-grow select-all normal-case truncate'),
          s($, 'class', 'py-2 px-1 bg-sky-500 hover:bg-sky-400 active:bg-sky-600'),
          s(a, 'class', 'flex bg-slate-700 rounded-xl w-96 mb-5 overflow-hidden'),
          s(C, 'class', 'text-md mb-2'),
          s(z, 'class', 'px-3 py-2 flex-grow select-all normal-case truncate'),
          s(H, 'class', 'py-2 px-1 bg-sky-500 hover:bg-sky-400 active:bg-sky-600'),
          s(b, 'class', 'flex bg-slate-700 rounded-xl w-96 mb-5 overflow-hidden'),
          s(U, 'for', 'num-words'),
          s(U, 'class', 'mb-2'),
          s(w, 'type', 'number'),
          s(w, 'name', 'num-words'),
          s(w, 'min', '1'),
          s(w, 'max', '6'),
          s(w, 'class', 'border-gray-300 p-2 rounded-lg block mt-1 bg-slate-700 w-48'),
          s(I, 'class', 'block mb-4'),
          s(D, 'for', 'num-words'),
          s(D, 'class', 'mb-2'),
          s(v, 'type', 'text'),
          s(v, 'name', 'separator'),
          s(v, 'maxlength', '1'),
          s(v, 'class', 'border-gray-300 p-2 rounded-lg block mt-1 bg-slate-700 w-48'),
          s(M, 'class', 'block mb-4'),
          s(ee, 'class', 'mr-2 align-bottom'),
          s(te, 'class', 'uppercase'),
          s(E, 'type', 'button'),
          s(
            E,
            'class',
            'flex items-center p-2 bg-slate-500 hover:bg-slate-400 active:bg-slate-600 rounded-xl'
          ),
          s(ne, 'class', 'mr-2 align-bottom'),
          s(re, 'class', 'uppercase'),
          s(N, 'type', 'button'),
          s(
            N,
            'class',
            'flex items-center p-2 bg-sky-500 hover:bg-sky-400 active:bg-sky-600 rounded-xl'
          ),
          s(L, 'class', 'mt-7 flex space-x-3 justify-center'),
          s(A, 'class', 'text-sm'),
          s(n, 'class', 'my-5 mx-6 font-mono'),
          s(e, 'class', 'absolute w-full h-full bg-slate-800 text-slate-50 uppercase')
      },
      m(u, j) {
        R(u, e, j),
          l(e, n),
          l(n, r),
          l(n, o),
          l(n, i),
          l(n, f),
          l(n, d),
          l(n, g),
          l(n, a),
          l(a, y),
          x.m(y, null),
          l(a, h),
          l(a, $),
          J(_, $, null),
          l(n, O),
          l(n, C),
          l(n, V),
          l(n, b),
          l(b, z),
          k.m(z, null),
          l(b, ve),
          l(b, H),
          J(S, H, null),
          l(n, xe),
          l(n, ke),
          l(n, Ce),
          l(n, A),
          l(A, I),
          l(I, U),
          l(I, Pe),
          l(I, w),
          ae(w, t[2].len),
          l(A, Ae),
          l(A, M),
          l(M, D),
          l(M, Ee),
          l(M, v),
          ae(v, t[2].separator),
          l(A, Ne),
          l(A, L),
          l(L, E),
          l(E, ee),
          J(B, ee, null),
          l(E, je),
          l(E, te),
          l(L, Re),
          l(L, N),
          l(N, ne),
          J(q, ne, null),
          l(N, Oe),
          l(N, re),
          (se = !0),
          ge ||
            ((Se = [
              F($, 'click', t[6]),
              F(H, 'click', t[7]),
              F(w, 'input', t[8]),
              F(v, 'input', t[9]),
              F(E, 'click', t[4]),
              F(N, 'click', t[10]),
            ]),
            (ge = !0))
      },
      p(u, [j]) {
        oe === (oe = Ie(u)) && x ? x.p(u, j) : (x.d(1), (x = oe(u)), x && (x.c(), x.m(y, null))),
          le === (le = Me(u)) && k ? k.p(u, j) : (k.d(1), (k = le(u)), k && (k.c(), k.m(z, null))),
          j & 4 && He(w.value) !== u[2].len && ae(w, u[2].len),
          j & 4 && v.value !== u[2].separator && ae(v, u[2].separator)
      },
      i(u) {
        se ||
          (W(_.$$.fragment, u),
          W(S.$$.fragment, u),
          W(B.$$.fragment, u),
          W(q.$$.fragment, u),
          (se = !0))
      },
      o(u) {
        ie(_.$$.fragment, u),
          ie(S.$$.fragment, u),
          ie(B.$$.fragment, u),
          ie(q.$$.fragment, u),
          (se = !1)
      },
      d(u) {
        u && P(e), x.d(), Q(_), k.d(), Q(S), Q(B), Q(q), (ge = !1), Y(Se)
      },
    }
  )
}
function Ge(t) {
  navigator.clipboard.writeText(t)
}
function wt(t, e, n) {
  let r
  const o = { len: 3, separator: '-' }
  let i, f
  const d = rt({ len: o.len, separator: o.separator })
  Qe(t, d, (C) => n(2, (r = C)))
  function g() {
    d.set(o), n(0, (i = void 0)), n(1, (f = void 0))
  }
  async function a() {
    console.log('Calling on-generate...')
    const { len: C, separator: V } = Je(d)
    console.log('len: ', C, 'separator: ', V)
    try {
      const b = await Ke('generate_password', { len: C, separator: V })
      console.log('Returned successfully: ', b), n(0, (i = b.password)), n(1, (f = b.hash))
    } catch (b) {
      console.error('Failed to generate password:', b)
    }
  }
  function y() {
    Ge(i), document.querySelector('#generated-password')
  }
  function h() {
    Ge(f)
  }
  function $() {
    ;(r.len = He(this.value)), d.set(r)
  }
  function _() {
    ;(r.separator = this.value), d.set(r)
  }
  return [
    i,
    f,
    r,
    d,
    g,
    a,
    y,
    h,
    $,
    _,
    async () => {
      await a()
    },
  ]
}
class yt extends me {
  constructor(e) {
    super(), pe(this, e, wt, _t, Z, {})
  }
}
new yt({ target: document.getElementById('app') })

<script lang="ts">
  import Icon from "./Icon.svelte";
  import Modal from "./Modal.svelte";
  import ThemeModal from "./ThemeModal.svelte";

  let menuOpen = $state(false);
  let themeOpen = $state(false);
  let menuButton = $state<HTMLButtonElement | undefined>();

  function openTheme() {
    menuOpen = false;
    themeOpen = true;
  }

  function closeAndRefocus() {
    menuOpen = false;
    themeOpen = false;
    menuButton?.focus();
  }
</script>

<header>
  <span class="title">rust-svelte-template</span>
  <button
    class="icon-btn"
    type="button"
    aria-label="メニュー"
    aria-expanded={menuOpen}
    bind:this={menuButton}
    onclick={() => (menuOpen = !menuOpen)}
  >
    <Icon name="menu" />
  </button>
</header>

{#if menuOpen}
  <Modal title="メニュー" onclose={closeAndRefocus}>
    <nav class="menu">
      <button
        class="menu-item"
        type="button"
        data-autofocus
        onclick={openTheme}
      >
        テーマ設定
      </button>
      <a class="menu-item" href="/" onclick={closeAndRefocus}>トップ</a>
    </nav>
  </Modal>
{/if}

{#if themeOpen}
  <ThemeModal onclose={closeAndRefocus} />
{/if}

<style lang="sass">
  header
    position: sticky
    top: 0
    z-index: 10
    display: flex
    align-items: center
    justify-content: space-between
    height: var(--header-h)
    padding: 0 var(--sp-3)
    background: var(--c-wash-base)
    border-bottom: 1px solid var(--c-border)

  .title
    font-size: var(--fs-md)
    font-weight: 500

  .menu
    display: flex
    flex-direction: column
    gap: var(--sp-2)

  .menu-item
    display: block
    width: 100%
    padding: var(--sp-2) var(--sp-3)
    border: 1px solid var(--c-border)
    border-radius: var(--radius-sm)
    background: var(--c-surface-raised)
    color: var(--c-on-surface)
    font-size: var(--fs-md)
    font-weight: 500
    text-align: left
    text-decoration: none
    cursor: pointer

    &:hover
      background: var(--c-hover-1)
</style>

<script lang="ts">
	import { onMount } from 'svelte';

	export const open = () => {
		is_open = true;
		modal_node.focus();
	};

	let is_open = false;
	let modal_node: HTMLDivElement;

	onMount(() => {
		// modal_node.addEventListener('focusin', () => {});

		// blur is triggered when focus is lost. similar to focusout
		modal_node.addEventListener('blur', () => {
			is_open = false;
		});

		window.addEventListener('keydown', (e) => {
			if (e.key === 'Escape') {
				modal_node.blur(); // focus out
			}
		});
	});
</script>

<div class="overlay" class:overlay-open={is_open}>
	<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
	<div bind:this={modal_node} class="modal" class:modal-open={is_open} tabindex="0">
		<p>Modal stuff here</p>
	</div>
</div>

<style lang="scss">
	.overlay {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		z-index: 2;

		display: flex;
		justify-content: center;
		align-items: center;
		pointer-events: none;

		opacity: 0;
		transition: all 0.4s ease-in-out;
	}

	.overlay-open {
		opacity: 1;
		pointer-events: all;
		background-color: rgba(100, 100, 100, 0.3);
		transition: all 0.4s ease-in-out;
	}

	.modal {
		position: fixed;
		top: 10vh;
		bottom: 10vh;

		width: 400px;
		max-height: 600px;

		background-color: rgba(0, 0, 0, 0);
		z-index: 3;
		padding: 2em;
		-webkit-backdrop-filter: blur(12px);
		backdrop-filter: blur(12px);
		opacity: 0;
		filter: blur(9px);

		border: 1px solid transparent;

		box-shadow: 0px 16px 24px 2px rgba(0, 0, 0, 0.5);
		border-radius: 16px;

		transition: all 0.4s ease-in-out;

		outline: none;

		&:focus {
			border: 1px solid rgba($color: #fff, $alpha: 0.35);
		}
	}

	.modal-open {
		opacity: 1;
		filter: blur(0px);
		transform: translateY(20px);
		background-color: rgba(0, 0, 0, 0.8);
		transition-property: opacity, filter, transform, background-color;
		transition-duration: 0.4s;
		transition-timing-function: ease-in-out;
	}
</style>

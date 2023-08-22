<script lang="ts">
	export let initial_state = false;

	let selected = initial_state;
	let down = false;

	export let on_click: (is_on_after: boolean) => void;

	export function click() {
		selected = !selected;
		on_click(selected);
	}

	export function press() {
		down = true;
	}

	export function release() {
		down = false;
		click();
	}

	export function is_selected() {
		return selected;
	}
	export function is_down() {
		return down;
	}

	// // Style props
</script>

<button class:selected on:click={click}>
	<slot />
</button>

<style lang="scss">
	button {
		align-items: center;
		justify-content: center;

		width: 24;
		height: 24;

		background-color: rgba(0, 0, 0, 0);
		color: #aaa;
		fill: rgba(0, 0, 0, 0);
		border-radius: 8px;
		border: 1px solid #cccccc33;
		// border: 1px solid transparent;

		margin: 2px;


		cursor: pointer;
		user-select: none;
		-webkit-user-select: none;
		-webkit-user-drag: none;

		transition-property: background-color, color, fill, transform;
		transition-duration: 0.1s;
		transition-timing-function: ease-in-out;

		&:hover {
			background-color: rgba(127, 127, 127, 0.1);
			color: #ddd;

			transition-property: background-color, color;
			transition-duration: 0.1s;
			transition-timing-function: ease-in-out;
		}

		&:active {
			background-color: rgba(127, 127, 127, 0.5);
			transform: scale(0.9);

			transition-property: background-color, transform;
			transition-duration: 0.1s;
			transition-timing-function: ease-in-out;
		}
	}

	button.selected {
		background-color: #ccc;
		color: #111;
		// color: rgba(255, 0, 115, 1.0);

		border: 1px solid #ccc;
		// fill: #ccc;

		transition-property: background-color, color;
		transition-duration: 0.1s;
		transition-timing-function: ease-in-out;


		&:hover {
			background-color: rgba(200, 200, 200, 0.8);
			color: #333;

			border: 1px solid #eee;


			transition-property: background-color, color;
			transition-duration: 0.1s;
			transition-timing-function: ease-in-out;
		}
	}

	button.down {
	}
</style>

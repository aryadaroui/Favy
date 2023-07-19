import { createEventDispatcher } from 'svelte';

export const dispatch = {
	subscribe(fn: CallableFunction) {
		fn(createEventDispatcher());
		
		// eslint-disable-next-line @typescript-eslint/no-empty-function
		return () => { };
	}
};
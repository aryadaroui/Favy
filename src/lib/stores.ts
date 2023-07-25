import { writable } from 'svelte/store';

export enum Sentiment {
	Hate = -1,
	Neutral = 0,
	Love = 1
}

export type PhotoName = string;
export type PhotoInfo = {
	rating: number,
	love: Sentiment,
};
export type Filter = {
	hate: {
		is_on: boolean,
		look_for: boolean,
	},
	rating: {
		is_on: boolean,
		look_for: number,
	},
	love: {
		is_on: boolean,
		look_for: boolean,
	},
};
export type Settings = {
	canvas: {
		transparency: number,
		visible: boolean,
	};
	toolbar: {
		transparency: number,
		visible: boolean,
	};
	reel_size: number,
};

const initial_settings: Settings = {
	canvas: {
		transparency: 0.5,
		visible: true,
	},
	toolbar: {
		transparency: 0.5,
		visible: true,
	},
	reel_size: 10
};
const initial_filter: Filter = {
	hate: {
		is_on: false,
		look_for: false,
	},
	rating: {
		is_on: false,
		look_for: 0,
	},
	love: {
		is_on: false,
		look_for: false,
	},
};

const initial_status = {
	reel: {
		idx: 0,
		len: 0,
	},
	page: {
		idx: 0,
		len: 0,
	},
	text: ' '
};


export const workspace_dir = writable('');
export const photo_names = writable([] as PhotoName[]);
export const photo_map = writable(new Map<PhotoName, PhotoInfo>());
export const current_photo = writable({ photo_name: '', idx: 0 });

export const filter = writable(initial_filter);
export const status = writable(initial_status);

export const settings = writable(initial_settings);

// export const cache
export interface FullDBType {
	faction: string;
	date: Date;
	price: null | number;
	id: number;
	image: string;
	owner: number;
	reason: null | string;
	driver: null | string;
	target_faction: null | string;
	status: number;
	type: number;
}

export interface SMGetItemsFull {
	id: number;
	owner: string;
	img_1: number;
	img_2: number | undefined;
	status: number;
	reason: string | undefined;
	type: number | undefined;
	price: number | undefined;
	faction: number;
	handled_by: string | undefined;
	driver: null | string;
	target_faction: null | string;
	date: Date;
	item_type: number;
}

export interface Logs {
	owner: string;
	item_id: number | undefined;
	item_type: number | undefined;
	action: string;
	faction: string;
	message: string | undefined;
	date: Date;
}

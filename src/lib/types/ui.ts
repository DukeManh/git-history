export type HistoryItem = {
	key: string;
	keyType: 'sha' | 'tag';
	message: string;
	url: string;
};

export type HistoryList = HistoryItem[];

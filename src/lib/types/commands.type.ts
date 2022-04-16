export type ReadRepo = readonly [string, string];

export type GetCommits = readonly string[];

export type GitShow = {
	readonly sha: string;
	readonly author: string;
	readonly author_email: string;
	readonly author_date: number;
	readonly commit_date: number;
	readonly parent_sha: string;
	readonly message: string;
};

export type GitShowDiff = string;

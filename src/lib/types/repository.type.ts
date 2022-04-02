export default interface Repository {
	name: string;
	fullName?: string;
	branch: string;
	localPath: string;
	owner?: GithubUser;
	commits: Commit[];
	createdAt?: Date;
	updatedAt?: Date;
	pushedAt?: Date;
	gitURL?: string;
	size?: number;
	license?: string;
}

export interface GithubUser {
	login: string;
	email: string;
}

export interface Commit {
	sha: string;
	message: string;
}
export interface Tag {
	tag: string;
	message: string;
}

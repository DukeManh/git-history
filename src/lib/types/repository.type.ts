export default interface Repository {
	name: string;
	fullName?: string;
	branch: string;
	localPath: string;
	owner?: GithubUser;
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

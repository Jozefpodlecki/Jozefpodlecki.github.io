export type Site = {
  displayName: string;
  path: string;
};

export type Profile = {
  fullName: string;
};

export interface IState {
  profile: Profile;
  sites: Site[];
}

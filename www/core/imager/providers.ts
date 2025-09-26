export abstract class ImagerProvider {
  public abstract prepareUserBaseImage(nickname: string): Promise<URL>;
}

export class HabbliveImagerProvider implements ImagerProvider {
  public constructor(private imagerUrl: string) {}

  public async prepareUserBaseImage(nickname: string): Promise<URL> {
    const url = new URL(this.imagerUrl);
    url.searchParams.set("user", nickname);
    return url;
  }
}

export class HabbletImagerProvider implements ImagerProvider {
  public constructor(
    private imagerUrl: string,
    private apiBaseUrl: string,
  ) {}

  public async prepareUserBaseImage(nickname: string): Promise<URL> {
    const userResponse = await fetch(`${this.apiBaseUrl}/player/${nickname}`);
    const user = (await userResponse.json()) as { figure: string };

    const url = new URL(this.imagerUrl);
    url.searchParams.set("figure", user.figure);
    return url;
  }
}

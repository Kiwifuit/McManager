# Setup
*Since CurseForge requires API keys, you will need to set one up*
- Create a new API key [here](https://legacy.curseforge.com/account/api-tokens)
  - You need to [sign into CurseForge](https://sso.curseforge.com/oidc/interaction/AWEcwsYmPigJbjmg6amG3) for this step
- In this directory, create a `.env` file with the following content:
  ```env
  CURSEFORGE_API_KEY=your-key-here
  ```
- Run the tests
  ```
  cargo test
  ```
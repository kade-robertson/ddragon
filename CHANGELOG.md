# Changelog

## [0.5.0](https://github.com/kade-robertson/ddragon/compare/v0.4.0...v0.5.0) (2023-02-02)


### Features

* derive partialeq and/or eq where applicable ([#28](https://github.com/kade-robertson/ddragon/issues/28)) ([de01faf](https://github.com/kade-robertson/ddragon/commit/de01faf3cb7eb610251ad81e2be8c88fb77e8657))

## [0.4.0](https://github.com/kade-robertson/ddragon/compare/v0.3.1...v0.4.0) (2023-02-02)


### ⚠ BREAKING CHANGES

* simplify featureset ([#24](https://github.com/kade-robertson/ddragon/issues/24))

### Features

* add an async API ([#22](https://github.com/kade-robertson/ddragon/issues/22)) ([4f00b9c](https://github.com/kade-robertson/ddragon/commit/4f00b9cfbe9828291121b14cf12528a1a97f9501))


### Code Refactoring

* simplify featureset ([#24](https://github.com/kade-robertson/ddragon/issues/24)) ([97c98cc](https://github.com/kade-robertson/ddragon/commit/97c98cc97a32b8320ffe25fcf81ac5af8c938c42))

## [0.3.1](https://github.com/kade-robertson/ddragon/compare/v0.3.0...v0.3.1) (2023-01-24)


### Bug Fixes

* move to sync-only cacache ([#20](https://github.com/kade-robertson/ddragon/issues/20)) ([5ba5171](https://github.com/kade-robertson/ddragon/commit/5ba51714ea2e481e57d3676bc639d5782e4040f2))

## [0.3.0](https://github.com/kade-robertson/ddragon/compare/v0.2.0...v0.3.0) (2023-01-23)


### ⚠ BREAKING CHANGES

* add single champion query ([#18](https://github.com/kade-robertson/ddragon/issues/18))
* image sprite property should just be string ([#13](https://github.com/kade-robertson/ddragon/issues/13))
* caching as default feature, as ureq middleware ([#10](https://github.com/kade-robertson/ddragon/issues/10))

### Features

* add single champion query ([#18](https://github.com/kade-robertson/ddragon/issues/18)) ([a9f79a2](https://github.com/kade-robertson/ddragon/commit/a9f79a275b37fa6b01c2ef7496f293e99dc5e0f1))
* caching as default feature, as ureq middleware ([#10](https://github.com/kade-robertson/ddragon/issues/10)) ([9abce52](https://github.com/kade-robertson/ddragon/commit/9abce523695ecec3ab0861fb188b938c0e5281b0))
* derive debug for all models ([#7](https://github.com/kade-robertson/ddragon/issues/7)) ([9b4aad7](https://github.com/kade-robertson/ddragon/commit/9b4aad7de26053fe261d58b3fe56fccf248948a5))
* support full champion data ([#17](https://github.com/kade-robertson/ddragon/issues/17)) ([2eaa347](https://github.com/kade-robertson/ddragon/commit/2eaa347e0f26f477c2b4ed5fc37ca133229b5be0))
* support maps ([#11](https://github.com/kade-robertson/ddragon/issues/11)) ([74d4ca1](https://github.com/kade-robertson/ddragon/commit/74d4ca12f9e365f7303ab457defb011524db0dda))
* support mission assets ([#12](https://github.com/kade-robertson/ddragon/issues/12)) ([6479c47](https://github.com/kade-robertson/ddragon/commit/6479c47097cbcfa8e6ce638942de45cb23906374))
* support profile icons ([#15](https://github.com/kade-robertson/ddragon/issues/15)) ([b0aef4e](https://github.com/kade-robertson/ddragon/commit/b0aef4eec1af1b8c1bc0169d63066ea395925473))
* support spell buffs ([#16](https://github.com/kade-robertson/ddragon/issues/16)) ([bca7c0b](https://github.com/kade-robertson/ddragon/commit/bca7c0bae4d854ba44275698969d4bf4883b6151))


### Bug Fixes

* box ureq error to reduce enum size ([#9](https://github.com/kade-robertson/ddragon/issues/9)) ([0f48023](https://github.com/kade-robertson/ddragon/commit/0f480230b4c78d8d1ccbbe0d4757a9fc4605b6ee))
* image sprite property should just be string ([#13](https://github.com/kade-robertson/ddragon/issues/13)) ([141527e](https://github.com/kade-robertson/ddragon/commit/141527ef0293b17d6fa5dfc5894146968fb89769))

## [0.2.0](https://github.com/kade-robertson/ddragon/compare/v0.1.0...v0.2.0) (2023-01-20)


### Features

* derive clone and copy where possible ([#5](https://github.com/kade-robertson/ddragon/issues/5)) ([e95395e](https://github.com/kade-robertson/ddragon/commit/e95395e283f8d2f6b646513d2550b23a0710b678))

## 0.1.0 (2023-01-19)


### ⚠ BREAKING CHANGES

* change champion image sprite enum values

### Features

* data fetching, champion deserializing ([0df2559](https://github.com/kade-robertson/ddragon/commit/0df2559c33b3bcf30c7e0d6cbfb0127c553889d8))
* start of client, fetch latest version ([7eb9db7](https://github.com/kade-robertson/ddragon/commit/7eb9db7caab241460d5bf789835c16342b303745))
* support caching, fix bad paths ([8e9a531](https://github.com/kade-robertson/ddragon/commit/8e9a531d59ceab7d5bf75bb6c5adf73922a3eb46))
* support challenges ([7121963](https://github.com/kade-robertson/ddragon/commit/71219638450b87f5a739ccb2b59a210e6e905f9b))
* support language translation data ([6e4fcab](https://github.com/kade-robertson/ddragon/commit/6e4fcab54b0ea834c29aa7f36a16fc8bf883a06c))
* support retrieving items ([820fb87](https://github.com/kade-robertson/ddragon/commit/820fb8709b9c2c8b8efc2d43ce27052dbb21361b))
* support runes ([8faaf87](https://github.com/kade-robertson/ddragon/commit/8faaf875405b3bda52e645c34721bc155af6c6da))
* support summoner spells ([a6f1b71](https://github.com/kade-robertson/ddragon/commit/a6f1b71f8d035618da7b9999be50281d1524d12a))


### Bug Fixes

* change champion image sprite enum values ([552c27d](https://github.com/kade-robertson/ddragon/commit/552c27db94571076df73e89e07ce91d0797115d4))
* replace anyhow with thiserror ([32d5120](https://github.com/kade-robertson/ddragon/commit/32d512088b364f003e62a0affb7e25519d422082))

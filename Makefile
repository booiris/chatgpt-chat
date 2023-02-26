project_name = app
passwd = 123456

gen_android:
	cargo tauri android build && cp -a src-tauri/gen/android/$(project_name)/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk ./a.apk && zipalign -f -v 4 a.apk out.apk && echo $(passwd) | apksigner sign --ks ./key/xample.keystore --ks-key-alias publishingdoc out.apk && rm a.apk && rm out.apk.idsig && mv out.apk ./out

get_cmd:
	yarn
	yarn upgrade @tauri-apps/cli@next @tauri-apps/api@next
	cd src-tauri && cargo add tauri@2.0.0-alpha.3  && cargo add tauri-build@2.0.0-alpha.1 --build
	cargo install tauri-cli --version "^2.0.0-alpha"
	cargo tauri android init
	# env: https://next--tauri.netlify.app/next/mobile/development/mobile_workflow
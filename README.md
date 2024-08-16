# AWS Lambda 用 VoicevoxCore

このリポジトリには、[VoicevoxCore](https://github.com/VOICEVOX/voicevox_core/) を AWS Lambda にデプロイするためのコードが含まれています。

## 使用方法

### 1. このリポジトリをクローンします。

### 2. 依存関係をインストールします。

```bash
npm install
```

### 3. Lambda 関数をデプロイします。

#### 3.1 (Option)

以下のファイルをダウンロードし、`PROJECT_ROOT/lambda-vvcore/cache` に配置しておくと、デプロイ時にダウンロードをスキップできます。

- [voicevox_core-linux-arm64-cpu-0.15.4.zip](https://github.com/VOICEVOX/voicevox_core/releases/download/0.15.4/voicevox_core-linux-arm64-cpu-0.15.4.zip)
- [open_jtalk_dic_utf_8-1.11.tar.gz](https://sourceforge.net/projects/open-jtalk/files/Dictionary/open_jtalk_dic-1.11/open_jtalk_dic_utf_8-1.11.tar.gz/download)

(しばしば)ダウンロードが失敗して Docker コンテナのビルドが失敗することに対するささやかな対策です。

#### 3.2

```bash
export CDK_DEFAULT_ACCOUNT=123456789012 （あなたの AWS アカウント ID）
export CDK_DEFAULT_REGION=ap-northeast-1 （あなたの AWS リージョン）
cdk deploy
```

### 4. Lambda 関数を呼び出します。

※ 以下のコードを実行するためには `jq` と `base64` がインストールされていることを確認してください。

```bash
export LAMBDA_ENDPOINT=https://YOUR-FUNCTION-URL.lambda-url.ap-northeast-1.on.aws/

JSON='{"text": "ようこそ、ボイスボックスへ！", "speaker_id": '0'}'
response=$(curl -XPOST "$LAMBDA_ENDPOINT" -H "Content-Type: application/json" -d "$JSON")
echo $response | jq -r '.wav' | base64 -d > "response.wav"
```

`response.wav` に音声ファイルが出力されます。

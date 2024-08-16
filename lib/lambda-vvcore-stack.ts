import { Stack, StackProps, Duration, CfnOutput } from "aws-cdk-lib";
import { Construct } from "constructs";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as ecr_assets from "aws-cdk-lib/aws-ecr-assets";
import * as path from "path";

export class LambdaVvcoreStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    // Create a Docker image asset from the Dockerfile
    const dockerImageAsset = new ecr_assets.DockerImageAsset(this, "LambdaVvcoreImage", {
      directory: path.join(__dirname, "..", "lambda-vvcore"),
      platform: ecr_assets.Platform.LINUX_ARM64,
    });

    // Create a Lambda function using the Docker image
    const lambdaFunction = new lambda.DockerImageFunction(this, "LambdaVvcoreFunction", {
      code: lambda.DockerImageCode.fromEcr(dockerImageAsset.repository, {
        tagOrDigest: dockerImageAsset.imageTag,
      }),
      architecture: lambda.Architecture.ARM_64,
      timeout: Duration.seconds(120),
      // https://qiita.com/takuma818t/items/a25e22fec1863707be08 (2020年情報)
      // 2047 MB だと 2vCPU になるらしい。遅い。こんにちは に 初期10秒、2回目以降でも7秒かかる。
      // 10240 MB だと 6vCPU になるらしい。速い。こんにちは に 初期5秒、2回目以降 1秒になる。
      memorySize: 10240,
      description: "Lambda function created from Docker image for lambda-vvcore",
      environment: {
        // Add the API key as an environment variable
        LAMBDA_APIKEY: process.env.LAMBDA_APIKEY || "",
      },
    });

    // Add Function URL to the Lambda function
    const functionUrl = lambdaFunction.addFunctionUrl({
      authType: lambda.FunctionUrlAuthType.NONE, // This makes the URL publicly accessible
      cors: {
        allowedOrigins: ["*"], // Allow requests from any origin
        allowedMethods: [lambda.HttpMethod.ALL], // Allow all HTTP methods
        allowedHeaders: ["*"], // Allow all headers
      },
    });

    // Output the Function URL
    new CfnOutput(this, "LambdaFunctionUrl", {
      value: functionUrl.url,
      description: "URL for the Lambda function",
      exportName: "LambdaVvcoreFunctionUrl",
    });
  }
}

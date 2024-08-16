#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { LambdaVvcoreStack } from "../lib/lambda-vvcore-stack";

const AWS_ACCOUNT = process.env.CDK_DEFAULT_ACCOUNT;
const AWS_REGION = process.env.CDK_DEFAULT_REGION;

const app = new cdk.App();
new LambdaVvcoreStack(app, "LambdaVvcoreStack", {
  env: {
    account: AWS_ACCOUNT,
    region: AWS_REGION,
  },
});

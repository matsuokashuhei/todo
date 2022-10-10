import { Authenticator } from "@aws-amplify/ui-react";
import { Amplify } from "aws-amplify";
import type { AppProps } from "next/app";
import "../styles/globals.css";

Amplify.configure({
  aws_project_region: process.env.NEXT_PUBLIC_AWS_REGION,
  aws_cognito_region: process.env.NEXT_PUBLIC_AWS_REGION,
  aws_user_pools_id: process.env.NEXT_PUBLIC_AWS_COGNITO_USER_POOL_ID,
  aws_user_pools_web_client_id:
    process.env.NEXT_PUBLIC_AWS_COGNITO_USER_POOL_CLIENT_ID,
  // ssr: true,
});

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <Authenticator>
      <Component {...pageProps} />
    </Authenticator>
  );
}

export default MyApp;

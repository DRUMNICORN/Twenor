import type { NextAuthOptions } from 'next-auth'
import GitHubProvider from 'next-auth/providers/github'
import CredentialsProvider from 'next-auth/providers/credentials'

import 'next-auth'

declare module 'next-auth' {
    interface User {
        accessToken?: string
    }

    interface Session {
        accessToken?: string
    }
}

export const options: NextAuthOptions = {
    providers: [
        GitHubProvider({
            clientId: process.env.GITHUB_CLIENT_ID as string,
            clientSecret: process.env.GITHUB_CLIENT_SECRET as string,
            // accessTokenUrl: 'http://127.0.0.1:8000/auth/login',
        }),
        CredentialsProvider({
            name: 'Credentials',
            credentials: {
                username: { label: 'Username', type: 'text' },
                password: { label: 'Password', type: 'password' },
            },
            async authorize(credentials) {
                let credentials_mapped = {
                    username: credentials?.username,
                    password: credentials?.password,
                };

                const res = await fetch('http://127.0.0.1:8000/auth/user', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(credentials_mapped),
                });
                const user = await res.json();
                if (res.ok && user) {
                    let token = user.token;
                    user.accessToken = token;
                    return user;
                }

                console.error('Auth failed', user);
                return null;
            }
        }),
    ],
    secret: process.env.SECRET,
    callbacks: {
        jwt: async ({ token, user, account, profile, isNewUser }) => {
            if (account?.provider === 'github' && account?.access_token) {
                token.accessToken = account.access_token;
            } else if (user?.accessToken) {
                token.accessToken = user.accessToken;
            }
            return token;
        },
        session: async ({ session, token }) => {
            if (token?.accessToken) {
                session.accessToken = token.accessToken as string;
            }
            // Ensure the token is still valid
            // const res = await fetch('http://127.0.0.1:8000/auth/login', {
            //     method: 'POST',
            //     headers: {
            //         'Content-Type': 'application/json',
            //         'Authorization': `Bearer ${token.accessToken}`,
            //     },
            // });
            // if (!res.ok) {
            //     throw new Error('Token validation failed');
            // }
            return session;
        }
    },
}

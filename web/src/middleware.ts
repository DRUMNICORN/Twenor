export { default } from 'next-auth/middleware'
export const config = { 
    matcher: ["/api/auth", "/api/auth/callback", "/api/auth/signin", "/api/auth/signout"]
}
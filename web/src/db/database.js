// import mysql from 'mysql';

// const dbConfig = {
//   host: 'localhost',
//   port: 3307,
//   user: 'root',
//   password: 'admin',
//   database: 'waitlist',
// };

// export const addUserToWaitlist = (userId, cardId) => {
//   const connection = mysql.createConnection(dbConfig);
//   connection.connect();

//   const query = `INSERT INTO waitlist (userId, cardId) VALUES (${userId}, ${cardId})`;

//   connection.query(query, (err, results) => {
//     if (err) {
//       console.error(err);
//     } else {
//       console.log('User added to waitlist');
//     }
//     connection.end();
//   });
// };

// export const removeUserFromWaitlist = (userId, cardId) => {
//   const connection = mysql.createConnection(dbConfig);
//   connection.connect();

//   const query = `DELETE FROM waitlist WHERE userId = ${userId} AND cardId = ${cardId}`;

//   connection.query(query, (err, results) => {
//     if (err) {
//       console.error(err);
//     } else {
//       console.log('User removed from waitlist');
//     }
//     connection.end();
//   });
// };

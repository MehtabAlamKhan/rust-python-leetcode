function a(target, array) {
  let dp = Array(target + 1).fill(false);
  dp[0] = true;

  for (let num of array) {
    if (dp[i] == true) {
      for (let i = 0; i <= amount; i++) {}
    }
  }

  for (let i = 0; i <= target; i++) {
    if (dp[i] == true) {
      for (let num of array) {
        dp[i + num] = true;
      }
    }
  }
  console.log(dp);
  return dp[target];
}

console.log(a(3, [7]));

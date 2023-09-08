class Solution {
    #define ll long long
public:
    int minSubArrayLen(int target, vector<int>& nums) {
        int n=nums.size();
        vector<ll>v(n+1,0);
        for(int i=0;i<n;i++){
            v[i+1]=v[i] + nums[i];
        }
        ll ans = INT_MAX;
       for(int i=0;i<n;i++){
           ll sm = target + v[i];
           ll in = lower_bound(v.begin(),v.end(),sm) - v.begin();
           if(in!=n+1)ans=min(ans,in-i);
       }
       if(ans==INT_MAX)ans=0;
       return ans;
    }
};
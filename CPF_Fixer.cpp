#include <bits/stdc++.h>
using namespace std;

void calculate(int list[]){
    int sum = 0;
    for(int i = 10; i > 1; i--){
        sum += i*list[10-i];
    }
    list[9] = 11-(sum%11);
    
    sum = 0;
    for(int i = 10; i > 1; i--){
        sum += i*list[10-i+1];
    }
    list[10] = 11-(sum%11);
    return;
}

void copy(int list1[], int list2[]){
    for(int i = 0; i < 11; i++){
        list2[i] = list1[i];
    }
    return;
}

int main()
{
    int list[11];
    
    for(int &i : list) cin >> i;
    
    for(int i = 0; i < 9; i++){
        for(int j = 0; j < 10; j++){
            int list2[11];
            copy(list, list2);
            list2[i] = j;
            
            calculate(list2);
            if(list2[9] == list[9] && list2[10] == list[10]){
                for(int k = 0; k < 11; k++){
                    cout << list2[k];
                }
                cout << '\n';
            }
        }
    }

    return 0;
}

package test;

import java.security.Provider;
import java.security.Security;
import java.util.Iterator;

public class Test {

    public final String hi = "";
    private int h = 1;
    private final int[][][] x = new int[0][0][0];

    public Test() {}

    public static void main(String[] args) {
        System.out.println("Hello");
    }

    public static Provider test(String[] var0) {
        Provider[] var1 = Security.getProviders();
        int var2 = var1.length;

        for(int var3 = 0; var3 < var2; ++var3) {
            Provider var4 = var1[var3];
            System.out.println(var4.getName());
            Iterator var5 = var4.stringPropertyNames().iterator();

            while(var5.hasNext()) {
                String var6 = (String)var5.next();
                System.out.println(var4.getProperty(var6));
            }
        }

        return var1[0];
    }
}
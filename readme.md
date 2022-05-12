# 玩具编译原理课设项目

## 语法制导翻译—逆波兰式的翻译

通过课程设计，理解掌握语法制导翻译的原理与应用，理解中间代码，掌握把表达式翻译出中间代码的算法与思想，并设计出错处理方法。

## 项目介绍

玩具编译器，用于完成编译原理课设，实现了如下内容：

-   [x] 词法分析
-   [x] 语法分析
-   [x] 错误输出
-   [x] AST 生成
-   [x] 逆波兰式生成
-   [x] 计算结果
-   [x] GUI 实现

输入按顺序经过了：

1. 词法分析
1. 语法分析
1. 中间代码生成
1. 逆波兰式生成
1. 计算并输出结果

一些特点：

1. 只承认所有直接跟着数字的 - 符号为负号，如 -5 ，不承认 -(5) 的形式。
1. 负号将以 @ 符号于逆波兰式中表现，以避免和减号 - 的歧义。
1. 允许输入科学记数法如 1e3 1.9E2 等。小数可简写为 .78 ，等价于 0.78 。
1. 内部计算使用精确数 (BigDecimal) 运算而不是浮点数，避免出现浮点错误和 int 溢出。
1. 运算精度为小数点后 2 的 64 次方 位，显示输出精度为小数点后 15 位有效数字。

## 软件截图

![](/doc/image/Screenshot_01.jpg)
![](/doc/image/Screenshot_02.jpg)
![](/doc/image/Screenshot_03.jpg)

## TODO

-   [ ] GUI 美化
-   [ ] 支持 `-(9)` `2+-(3+2)` 这样的表达式
-   [ ] 支持剔除 `1+-2` 这样的，只支持 `1+(-2)` 这样的

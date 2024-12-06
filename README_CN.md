# HW_DCMI

华为昇腾计算设备**第三方**DCMI c 库 **safe** FFI绑定

- hw_dcmi提供safe的FFI绑定(由hw_dcmi_sys提供的FFI绑定封装而成)
- hw_dcmi_sys提供unsafe的FFI绑定(由bindgen直接生成)

## 使用方法
### 先决条件
项目在Ubuntu 22.04上测试过，使用Atlas 6.0.0 DCMI API，你需要安装以下依赖项:

- DCMI共享库
- 昇腾驱动

默认情况下，库将尝试在`/usr/local/dcmi`中查找`dcmi_interface_api.h`并链接`libdcmi.so`，
你可以提供`HW_DCMI_PATH`环境变量来指定dcmi共享库的路径。


## **项目状态: 进行中**
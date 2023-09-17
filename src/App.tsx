import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { message, Form, Switch } from 'antd';
import { SWITCH_COMMAND } from './const';
import './App.css';
function App() {
  const [form] = Form.useForm();
  const [isOpen, setIsOpen] = useState<boolean>(false);

  /**
   * @description 开启监听
   */
  const onChangeIsOpen = async (checked: boolean) => {
    try {
      setIsOpen(checked);
      const res: string = await invoke(SWITCH_COMMAND, { switchValue: checked });
      message.info(`操作成功, ${res}`);
    } catch (error) {
      message.error(`操作成功, ${JSON.stringify(error)}`);
      setIsOpen(!checked);
    }
  }

  return (
    <>
      <h1>峡谷钢琴家1.0</h1>
      <div className="card">
        <Form
          labelCol={{ span: 0 }}
          wrapperCol={{ span: 24 }}
          form={form}
          style={{ minWidth: 200, maxWidth: 600 }}
        >
          <Form.Item>
            <Switch
              checked={isOpen}
              checkedChildren="开启"
              unCheckedChildren="关闭"
              onChange={onChangeIsOpen}
            />
          </Form.Item>
        </Form>
      </div>
    </>
  )
}

export default App

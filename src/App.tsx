import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { message, Form, Switch, Input } from 'antd';
import { SWITCH_COMMAND } from './const';
import './App.css';

function App() {
  const [form] = Form.useForm();
  const [inputText, setInputText] = useState<string>('');
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
    <div className="container">
      <h1>峡谷钢琴家1.0</h1>
      <Form
        labelCol={{ span: 0 }}
        wrapperCol={{ span: 24 }}
        form={form}
        className="form"
      >
        <Form.Item>
          <Input.TextArea
            rows={4}
            value={inputText}
            onChange={(e) => { setInputText(e.target.value) }}
          />
        </Form.Item>
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
  );
}

export default App

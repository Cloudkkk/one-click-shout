import { useRef, useState } from 'react';
import { emit, listen } from '@tauri-apps/api/event';
import { message, Form, Switch, Select, Input } from 'antd';
import { USERKEYCHANNEL, PRESSCHANNEL } from './const';
import './App.css';

function App() {
  const listener = useRef(null);
  const [form] = Form.useForm();
  const [selectedItem, setSelectedItem] = useState('');
  const [textInput, setTextInput] = useState('');
  const [isOpen, setIsOpen] = useState(false);

  const TemplteOptions = [
    { label: '模板1', value: '模板1' },
    { label: '模板2', value: '模板2' },
    { label: '模板3', value: '模板3' }
  ]

  const onChangeSelect = async (value) => {
    setSelectedItem(value);
    onCloseListen();
    await emit(USERKEYCHANNEL, {
      user_key: selectedItem
    })
  }

  /**
   * @description 开启监听
   */
  const onOpenListen = async () => {
    try {
      await form.validateFields();
      if (!listener.current) {
        message.success('开始你的表演');
        (async () => {
          listener.current = await listen(PRESSCHANNEL, (event) => {
            console.log('1111');
          });
        })();
      }
    } catch (error) {
      console.log(error);
      setIsOpen(false);
    }
  }

  /**
   * @description 关闭监听
   */
  const onCloseListen = () => {
    setIsOpen(false);
    if (listener.current) {
      message.info('演出结束了');
      listener.current();
      listener.current = null;
    }
  }

  return (
    <>
      <h1>Vite + React</h1>
      <div className="card">
        <Form
          labelCol={{ span: 0 }}
          wrapperCol={{ span: 24 }}
          form={form}
          style={{ minWidth: 200, maxWidth: 600 }}
        >
          <Form.Item name='select' rules={[{ required: true, message: '请选择模式' }]}>
            <Select
              value={selectedItem}
              options={TemplteOptions}
              onChange={onChangeSelect}
            />
          </Form.Item>
          <Form.Item name='input' rules={[{ required: true, message: '请输入内容' }]}>
            <Input.TextArea
              value={textInput}
              rows={4}
              onInput={(e) => { setTextInput(e.target.value); onCloseListen(); }}
            />
          </Form.Item>
          <Form.Item>
            <Switch
              checked={isOpen}
              checkedChildren="开启"
              unCheckedChildren="关闭"
              onChange={(checked) => { setIsOpen(checked); checked ? onOpenListen() : onCloseListen(); }}
            />
          </Form.Item>
        </Form>
      </div>
    </>
  )
}

export default App

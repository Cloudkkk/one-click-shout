import { useRef, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import { message, Form, Switch, Select, Input } from 'antd';
import './App.css';

function App() {
  const listener = useRef(null);
  const [form] = Form.useForm();
  const [isAuto, setIsAuto] = useState(true);
  const [selectedItem, setSelectedItem] = useState('');
  const [textInput, setTextInput] = useState('');
  const [isOpen, setIsOpen] = useState(false);

  const OPTIONS = [
    { label: '按A', value: 'A' },
    { label: '按S', value: 'S' },
    { label: '按D', value: 'D' }
  ]

  /**
   * @description 开启监听
   */
  const onOpenListen = async () => {
    try {
      await form.validateFields();
      if (!listener.current) {
        message.success('开始你的表演');
        (async () => {
          listener.current = await listen('f1_pressed', (event) => {
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
          <Form.Item>
            <div>
              <span className='mr10'>自己动</span>
              <Switch
                checked={isAuto}
                onChange={(checked) => { setIsAuto(checked); onCloseListen(); }}
              />
              <span className='ml10'>全自动</span>
            </div>
          </Form.Item>
          {
            isAuto ? (
              <Form.Item name='select' rules={[{ required: true, message: '请选择模式' }]}>
                <Select
                  value={selectedItem}
                  options={OPTIONS}
                  onChange={(value) => { setSelectedItem(value); onCloseListen(); }}
                />
              </Form.Item>
            ) : (
              <Form.Item name='input' rules={[{ required: true, message: '请输入内容' }]}>
                <Input.TextArea
                  value={textInput}
                  rows={4}
                  onInput={(e) => { setTextInput(e.target.value); onCloseListen(); }}
                />
              </Form.Item>
            )
          }
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

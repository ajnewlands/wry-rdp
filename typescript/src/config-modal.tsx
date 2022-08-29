import Button from 'react-bootstrap/Button';
import Modal from 'react-bootstrap/Modal';
import React from 'react';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';

function ConfigModal() {
    const [show, setShow] = React.useState(true);
    const [port, setPort] = React.useState(3389);
    const [host, setHost] = React.useState("127.0.0.1");
    const [user, setUser] = React.useState("");
    const [password, setPassword] = React.useState("");


    return (
      <Modal show={show} onHide={()=>setShow(false)}>
        <Modal.Header closeButton>
          <Modal.Title>Connection Settings</Modal.Title>
        </Modal.Header>

        <Modal.Body>
            <Row className="my-1">
                <Col xs={4}>
                <span>Username</span>
                </Col>
                <Col xs={8}>
                <input value={user} className="w-100" onChange={e=>setUser(e.target.value)}></input>
                </Col>
            </Row>
            <Row className="my-1">
                <Col xs={4}>
                <span>Password</span>
                </Col>
                <Col xs={8}>
                <input value={password} className="w-100" onChange={e=>setPassword(e.target.value)}></input>
                </Col>
            </Row>
            <Row className="my-1">
                <Col xs={4}>
                <span>Hostname</span>
                </Col>
                <Col xs={8}>
                <input value={host} className="w-100" onChange={e=>setHost(e.target.value)}></input>
                </Col>
            </Row>
            <Row className="my-1">
                <Col xs={4}>
                <span>Port</span>
                </Col>
                <Col xs={8}>
                <input value={port} className="w-100" onChange={e=>setPort(Number.parseInt(e.target.value))}></input>
                </Col>
            </Row>
        </Modal.Body>

        <Modal.Footer>
          <Button variant="secondary" onClick={() => setShow(false)}>Close</Button>
          <Button variant="primary">Save changes</Button>
        </Modal.Footer>
      </Modal>
  );
}

export default ConfigModal;
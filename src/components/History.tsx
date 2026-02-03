export function History() {
  return (
    <>
      <div className="main-header">
        <h2>历史记录</h2>
        <div style={{ display: 'flex', gap: '8px' }}>
          <input
            type="text"
            placeholder="搜索会话..."
            className="btn"
            style={{ textAlign: 'left', width: '240px', cursor: 'text' }}
          />
          <button className="btn btn-primary">搜索</button>
        </div>
      </div>
      <div className="content-scroll">
      <div className="empty-state">
        <div className="empty-state-icon">◷</div>
        <h3>历史记录功能</h3>
        <p>启用 Hook 后将自动记录会话历史</p>
      </div>
      </div>
    </>
  )
}

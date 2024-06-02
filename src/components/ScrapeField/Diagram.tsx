import { observer, useObservable, useObserve } from '@legendapp/state/react'
import { useCallback } from 'react'
import ReactFlow, { addEdge, Background, useNodesState, useEdgesState, Controls } from 'reactflow'
import 'reactflow/dist/style.css'
import { Box, Flex} from '@radix-ui/themes'
import { TaskQueue, TQTask } from '../..'
import { taskQueue } from '../../core/state/taskQueue'
import dagre from 'dagre'
import { initialEdges, initialNodes } from '../../core/state'

const dagreGraph = new dagre.graphlib.Graph()
dagreGraph.setDefaultEdgeLabel(() => ({}))

const nodeWidth = 172
const nodeHeight = 36

const getLayoutedElements = (nodes, edges, direction = 'TB') => {
  const isHorizontal = direction === 'LR'
  dagreGraph.setGraph({ rankdir: direction })

  nodes.forEach((node) => {
    dagreGraph.setNode(node.id, { width: nodeWidth, height: nodeHeight })
  })

  edges.forEach((edge) => {
    dagreGraph.setEdge(edge.source, edge.target)
  })

  dagre.layout(dagreGraph)

  nodes.forEach((node) => {
    const nodeWithPosition = dagreGraph.node(node.id)
    node.targetPosition = isHorizontal ? 'left' : 'top'
    node.sourcePosition = isHorizontal ? 'right' : 'bottom'

    // We are shifting the dagre node position (anchor=center center) to the top left
    // so it matches the React Flow node anchor point (top left).
    node.position = {
      x: nodeWithPosition.x - nodeWidth / 2,
      y: nodeWithPosition.y - nodeHeight / 2
    }

    // const l = nodes.find((t) => t.id === 'q')
    // if (l) {
    //   // l.type = 'group'
    // }

    return node
  })

  return { nodes, edges }
}

const { nodes: layoutedNodes, edges: layoutedEdges } = getLayoutedElements(
  initialNodes,
  initialEdges
)

const addNodes = (tq: TaskQueue) => {
  const nodes: any[] = []
  const edges: any[] = []
  for (const tqType in tq) {
    // @ts-ignore
    tq[tqType].forEach((task: TQTask) => {
      const type = task.task_type === 'enqueue' ? 'q' : task.task_type === 'processing' ? 'p' : 't'

      nodes.push({
        id: task.task_id,
        position: { x: 10, y: 10 },
        extent: 'parent',
        data: { label: task.action_data.task_group },
        className: 'light',
        parentId: type
      })
    })
  }
  return [nodes, edges]
}

export const Diagram = observer(() => {
  const [nodes, setNodes, onNodesChange] = useNodesState(layoutedNodes)
  const [edges, setEdges, onEdgesChange] = useEdgesState(layoutedEdges)

  const onConnect = useCallback(
    (params) => setEdges((eds) => addEdge({ ...params, animated: true }, eds)),
    []
  )

  useObserve(() => {
    const [newTNodes, newTEdges] = addNodes(taskQueue.get())
    setNodes([...nodes, ...newTNodes])
    setEdges([...edges, ...newTEdges])
  })

  return (
    <Flex>
      <Box className="w-[25rem] h-[15rem]">
        <ReactFlow
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
          className="bg-white"
          fitView
        >
          <Controls />
          <Background />
        </ReactFlow>
      </Box>
    </Flex>
  )
})
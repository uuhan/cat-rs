#include "client.h"

void test() {
    /**
     * 如果当前的栈为空，一个消息树会被创建
     */
    CatTransaction *t1 = newTransaction("foo", "bar1");

    /**
     * Metric 可以在任何地方记录，并不会被加到消息树中
     */
    logMetricForCount("metric-count", 1);
    logMetricForDuration("metric-duration", 200);

    /**
     * 记录一个给定耗时的 transaction 并立刻完成它。
     */
    newCompletedTransactionWithDuration("foo", "bar2-completed", 1000);

    /**
     * Transaction can be nested.
     * Transaction 可以嵌套，最新的 transaction 会被推到栈顶
     */
    CatTransaction *t3 = newTransaction("foo", "bar3");
    t3->setStatus(t3, CAT_SUCCESS);
    /**
     * 当你完成一个 transaction 的时候，它会被从栈里面弹出，并且 duration 会被计算。
     */
    t3->complete(t3);

    char buf[10];
    for (int k = 0; k < 3; k++) {
        /**
         * 创建一个给定耗时的 transaction
         */
        CatTransaction *t4 = newTransactionWithDuration("foo", "bar4-with-duration", 1000);
        snprintf(buf, 9, "bar%d", k);
        /**
         * 记录一个 event，会被添加到当前 transaction 的儿子列表中
         */
        logEvent("foo", buf, CAT_SUCCESS, 0);
        t4->setStatus(t4, CAT_SUCCESS);
        t4->complete(t4);
    }

    t1->setStatus(t1, CAT_SUCCESS);
    /**
     * 完成 transaction 并将它从栈里弹出
     * 当最后一个元素被弹出时，消息树会被序列化并发送给服务端
     */
    t1->complete(t1);
}

int main(int argc, char **argv) {
    CatClientConfig config = DEFAULT_CCAT_CONFIG;
    config.enableHeartbeat = 0;
    config.enableDebugLog = 1;
    catClientInitWithConfig("ccat", &config);
    catClientDestroy();
    return 0;
}

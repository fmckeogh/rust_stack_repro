#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use Mem_read__2::*;
use u__id::*;
use common::*;
pub fn Mem_read__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    size: i128,
    accdesc: ProductType9878976b5bcce9c9,
    ispair: bool,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        gs_20266: bool,
        gs_20270: bool,
        gs_20268: bool,
        ga_15709: Bits,
        gs_20264: bool,
        address: u64,
        size: i128,
        accdesc: ProductType9878976b5bcce9c9,
        ispair: bool,
    }
    let fn_state = FunctionState {
        address,
        size,
        accdesc,
        ispair,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var size:i
        let s_0_0: i128 = fn_state.size;
        // D s_0_1: call __id(s_0_0)
        let s_0_1: i128 = u__id(state, tracer, s_0_0);
        // C s_0_2: const #1s : i
        let s_0_2: i128 = 1;
        // D s_0_3: cmp-eq s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) == (s_0_2));
        // N s_0_4: branch s_0_3 b13 b1
        if s_0_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_1_0: read-var size:i
        let s_1_0: i128 = fn_state.size;
        // D s_1_1: call __id(s_1_0)
        let s_1_1: i128 = u__id(state, tracer, s_1_0);
        // C s_1_2: const #2s : i
        let s_1_2: i128 = 2;
        // D s_1_3: cmp-eq s_1_1 s_1_2
        let s_1_3: bool = ((s_1_1) == (s_1_2));
        // D s_1_4: write-var gs#20264 <= s_1_3
        fn_state.gs_20264 = s_1_3;
        // N s_1_5: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_2_0: read-var gs#20264:u8
        let s_2_0: bool = fn_state.gs_20264;
        // N s_2_1: branch s_2_0 b12 b3
        if s_2_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_3_0: read-var size:i
        let s_3_0: i128 = fn_state.size;
        // D s_3_1: call __id(s_3_0)
        let s_3_1: i128 = u__id(state, tracer, s_3_0);
        // C s_3_2: const #4s : i
        let s_3_2: i128 = 4;
        // D s_3_3: cmp-eq s_3_1 s_3_2
        let s_3_3: bool = ((s_3_1) == (s_3_2));
        // D s_3_4: write-var gs#20266 <= s_3_3
        fn_state.gs_20266 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_4_0: read-var gs#20266:u8
        let s_4_0: bool = fn_state.gs_20266;
        // N s_4_1: branch s_4_0 b11 b5
        if s_4_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_5_0: read-var size:i
        let s_5_0: i128 = fn_state.size;
        // D s_5_1: call __id(s_5_0)
        let s_5_1: i128 = u__id(state, tracer, s_5_0);
        // C s_5_2: const #8s : i
        let s_5_2: i128 = 8;
        // D s_5_3: cmp-eq s_5_1 s_5_2
        let s_5_3: bool = ((s_5_1) == (s_5_2));
        // D s_5_4: write-var gs#20268 <= s_5_3
        fn_state.gs_20268 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_6_0: read-var gs#20268:u8
        let s_6_0: bool = fn_state.gs_20268;
        // N s_6_1: branch s_6_0 b10 b7
        if s_6_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_7_0: read-var size:i
        let s_7_0: i128 = fn_state.size;
        // D s_7_1: call __id(s_7_0)
        let s_7_1: i128 = u__id(state, tracer, s_7_0);
        // C s_7_2: const #16s : i
        let s_7_2: i128 = 16;
        // D s_7_3: cmp-eq s_7_1 s_7_2
        let s_7_3: bool = ((s_7_1) == (s_7_2));
        // D s_7_4: write-var gs#20270 <= s_7_3
        fn_state.gs_20270 = s_7_3;
        // N s_7_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_8_0: read-var gs#20270:u8
        let s_8_0: bool = fn_state.gs_20270;
        // N s_8_1: assert s_8_0
        let s_8_1: () = assert!(s_8_0);
        // D s_8_2: read-var size:i
        let s_8_2: i128 = fn_state.size;
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // D s_8_4: read-var address:u64
        let s_8_4: u64 = fn_state.address;
        // D s_8_5: read-var accdesc:struct
        let s_8_5: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_8_6: read-var ispair:u8
        let s_8_6: bool = fn_state.ispair;
        // C s_8_7: const #0u : u8
        let s_8_7: bool = false;
        // D s_8_8: call Mem_read__2(s_8_4, s_8_3, s_8_5, s_8_6, s_8_7)
        let s_8_8: Bits = Mem_read__2(state, tracer, s_8_4, s_8_3, s_8_5, s_8_6, s_8_7);
        // D s_8_9: write-var ga#15709 <= s_8_8
        fn_state.ga_15709 = s_8_8;
        // N s_8_10: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_9_0: read-var ga#15709:bv
        let s_9_0: Bits = fn_state.ga_15709;
        // N s_9_1: return s_9_0
        return s_9_0;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#20270 <= s_10_0
        fn_state.gs_20270 = s_10_0;
        // N s_10_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#20268 <= s_11_0
        fn_state.gs_20268 = s_11_0;
        // N s_11_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#20266 <= s_12_0
        fn_state.gs_20266 = s_12_0;
        // N s_12_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#20264 <= s_13_0
        fn_state.gs_20264 = s_13_0;
        // N s_13_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}

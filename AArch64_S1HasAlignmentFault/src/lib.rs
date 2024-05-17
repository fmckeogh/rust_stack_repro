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
use ConstrainUnpredictable::*;
use HaveMTEExt::*;
use common::*;
pub fn AArch64_S1HasAlignmentFault<T: Tracer>(
    state: &mut State,
    tracer: &T,
    accdesc: ProductType9878976b5bcce9c9,
    aligned: bool,
    ntlsmd: bool,
    memattrs: ProductTypef170cab34335b70c,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_17415: bool,
        gs_17417: bool,
        gs_17414: bool,
        return_value: bool,
        gs_17416: bool,
        gs_17419: bool,
        gs_17418: bool,
        accdesc: ProductType9878976b5bcce9c9,
        aligned: bool,
        ntlsmd: bool,
        memattrs: ProductTypef170cab34335b70c,
    }
    let fn_state = FunctionState {
        accdesc,
        aligned,
        ntlsmd,
        memattrs,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var accdesc.1:struct
        let s_0_0: u32 = fn_state.accdesc._1;
        // C s_0_1: const #0u : u32
        let s_0_1: u32 = 0;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b27 b1
        if s_0_2 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HaveMTEExt(s_1_0)
        let s_1_1: bool = HaveMTEExt(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b26 b2
        if s_1_1 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#17414 <= s_2_0
        fn_state.gs_17414 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#17414:u8
        let s_3_0: bool = fn_state.gs_17414;
        // N s_3_1: branch s_3_0 b25 b4
        if s_3_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#17415 <= s_4_0
        fn_state.gs_17415 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#17415:u8
        let s_5_0: bool = fn_state.gs_17415;
        // N s_5_1: branch s_5_0 b21 b6
        if s_5_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var accdesc.0:struct
        let s_6_0: bool = fn_state.accdesc._0;
        // N s_6_1: branch s_6_0 b20 b7
        if s_6_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#17416 <= s_7_0
        fn_state.gs_17416 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var gs#17416:u8
        let s_8_0: bool = fn_state.gs_17416;
        // N s_8_1: branch s_8_0 b16 b9
        if s_8_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var accdesc.1:struct
        let s_9_0: u32 = fn_state.accdesc._1;
        // C s_9_1: const #7u : u32
        let s_9_1: u32 = 7;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // N s_9_3: branch s_9_2 b15 b10
        if s_9_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var memattrs.2:struct
        let s_10_0: u32 = fn_state.memattrs._2;
        // C s_10_1: const #1u : u32
        let s_10_1: u32 = 1;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // N s_10_3: branch s_10_2 b14 b11
        if s_10_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#17417 <= s_11_0
        fn_state.gs_17417 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var gs#17417:u8
        let s_12_0: bool = fn_state.gs_17417;
        // D s_12_1: write-var return_value <= s_12_0
        fn_state.return_value = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var return_value:u8
        let s_13_0: bool = fn_state.return_value;
        // N s_13_1: return s_13_0
        return s_13_0;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var aligned:u8
        let s_14_0: bool = fn_state.aligned;
        // D s_14_1: not s_14_0
        let s_14_1: bool = !s_14_0;
        // D s_14_2: write-var gs#17417 <= s_14_1
        fn_state.gs_17417 = s_14_1;
        // N s_14_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var memattrs.2:struct
        let s_15_0: u32 = fn_state.memattrs._2;
        // C s_15_1: const #1u : u32
        let s_15_1: u32 = 1;
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // D s_15_3: write-var return_value <= s_15_2
        fn_state.return_value = s_15_2;
        // N s_15_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var memattrs.2:struct
        let s_16_0: u32 = fn_state.memattrs._2;
        // C s_16_1: const #1u : u32
        let s_16_1: u32 = 1;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // N s_16_3: branch s_16_2 b19 b17
        if s_16_2 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#17418 <= s_17_0
        fn_state.gs_17418 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var gs#17418:u8
        let s_18_0: bool = fn_state.gs_17418;
        // D s_18_1: write-var return_value <= s_18_0
        fn_state.return_value = s_18_0;
        // N s_18_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_19_0: read-var memattrs.0:struct
        let s_19_0: u32 = fn_state.memattrs._0;
        // C s_19_1: const #0u : u32
        let s_19_1: u32 = 0;
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // D s_19_3: write-var gs#17418 <= s_19_2
        fn_state.gs_17418 = s_19_2;
        // N s_19_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var ntlsmd:u8
        let s_20_0: bool = fn_state.ntlsmd;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 1u16);
        // C s_20_2: const #0u : u8
        let s_20_2: bool = false;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 1u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: write-var gs#17416 <= s_20_4
        fn_state.gs_17416 = s_20_4;
        // N s_20_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var memattrs.2:struct
        let s_21_0: u32 = fn_state.memattrs._2;
        // C s_21_1: const #1u : u32
        let s_21_1: u32 = 1;
        // D s_21_2: cmp-eq s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) == (s_21_1));
        // N s_21_3: branch s_21_2 b24 b22
        if s_21_2 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#17419 <= s_22_0
        fn_state.gs_17419 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var gs#17419:u8
        let s_23_0: bool = fn_state.gs_17419;
        // D s_23_1: write-var return_value <= s_23_0
        fn_state.return_value = s_23_0;
        // N s_23_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_24_0: const #18u : u32
        let s_24_0: u32 = 18;
        // S s_24_1: call ConstrainUnpredictable(s_24_0)
        let s_24_1: u32 = ConstrainUnpredictable(state, tracer, s_24_0);
        // C s_24_2: const #12u : u32
        let s_24_2: u32 = 12;
        // S s_24_3: cmp-eq s_24_1 s_24_2
        let s_24_3: bool = ((s_24_1) == (s_24_2));
        // D s_24_4: write-var gs#17419 <= s_24_3
        fn_state.gs_17419 = s_24_3;
        // N s_24_5: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_25_0: read-var accdesc.32:struct
        let s_25_0: bool = fn_state.accdesc._32;
        // D s_25_1: write-var gs#17415 <= s_25_0
        fn_state.gs_17415 = s_25_0;
        // N s_25_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_26_0: read-var accdesc.27:struct
        let s_26_0: bool = fn_state.accdesc._27;
        // D s_26_1: write-var gs#17414 <= s_26_0
        fn_state.gs_17414 = s_26_0;
        // N s_26_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var return_value <= s_27_0
        fn_state.return_value = s_27_0;
        // N s_27_2: jump b13
        return block_13(state, tracer, fn_state);
    }
}

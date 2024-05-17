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
use u_get_SCR_EL3_Type_GPF::*;
use HaveRME::*;
use common::*;
pub fn ReportAsGPCException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault: ProductType1d757adad216cdef,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        ga_7136: ProductType396b95aa74979079,
        ga_7140: u32,
        gs_9887: bool,
        gs_9879: bool,
        ga_7139: ProductType396b95aa74979079,
        ga_7145: bool,
        return_value: bool,
        fault: ProductType1d757adad216cdef,
    }
    let fn_state = FunctionState {
        fault,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveRME(s_0_0)
        let s_0_1: bool = HaveRME(state, tracer, s_0_0);
        // N s_0_2: assert s_0_1
        let s_0_2: () = assert!(s_0_1);
        // D s_0_3: read-var fault.16:struct
        let s_0_3: u32 = fn_state.fault._16;
        // C s_0_4: const #12u : u32
        let s_0_4: u32 = 12;
        // D s_0_5: cmp-eq s_0_3 s_0_4
        let s_0_5: bool = ((s_0_3) == (s_0_4));
        // N s_0_6: branch s_0_5 b15 b1
        if s_0_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var fault.16:struct
        let s_1_0: u32 = fn_state.fault._16;
        // C s_1_1: const #13u : u32
        let s_1_1: u32 = 13;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // D s_1_3: write-var gs#9879 <= s_1_2
        fn_state.gs_9879 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#9879:u8
        let s_2_0: bool = fn_state.gs_9879;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // D s_2_2: read-var fault.6:struct
        let s_2_2: ProductType396b95aa74979079 = fn_state.fault._6;
        // D s_2_3: write-var ga#7136 <= s_2_2
        fn_state.ga_7136 = s_2_2;
        // D s_2_4: read-var ga#7136.0:struct
        let s_2_4: u32 = fn_state.ga_7136._0;
        // C s_2_5: const #0u : u32
        let s_2_5: u32 = 0;
        // D s_2_6: cmp-eq s_2_4 s_2_5
        let s_2_6: bool = ((s_2_4) == (s_2_5));
        // N s_2_7: assert s_2_6
        let s_2_7: () = assert!(s_2_6);
        // D s_2_8: read-var fault.6:struct
        let s_2_8: ProductType396b95aa74979079 = fn_state.fault._6;
        // D s_2_9: write-var ga#7139 <= s_2_8
        fn_state.ga_7139 = s_2_8;
        // D s_2_10: read-var ga#7139.0:struct
        let s_2_10: u32 = fn_state.ga_7139._0;
        // D s_2_11: write-var ga#7140 <= s_2_10
        fn_state.ga_7140 = s_2_10;
        // C s_2_12: const #2u : u32
        let s_2_12: u32 = 2;
        // D s_2_13: read-var ga#7140:u32
        let s_2_13: u32 = fn_state.ga_7140;
        // D s_2_14: cmp-eq s_2_12 s_2_13
        let s_2_14: bool = ((s_2_12) == (s_2_13));
        // D s_2_15: not s_2_14
        let s_2_15: bool = !s_2_14;
        // N s_2_16: branch s_2_15 b5 b3
        if s_2_15 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var return_value <= s_3_0
        fn_state.return_value = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var return_value:u8
        let s_4_0: bool = fn_state.return_value;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #1u : u32
        let s_5_0: u32 = 1;
        // D s_5_1: read-var ga#7140:u32
        let s_5_1: u32 = fn_state.ga_7140;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b7 b6
        if s_5_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var return_value <= s_6_0
        fn_state.return_value = s_6_0;
        // N s_6_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #3u : u32
        let s_7_0: u32 = 3;
        // D s_7_1: read-var ga#7140:u32
        let s_7_1: u32 = fn_state.ga_7140;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b9 b8
        if s_7_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var return_value <= s_8_0
        fn_state.return_value = s_8_0;
        // N s_8_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #4u : u32
        let s_9_0: u32 = 4;
        // D s_9_1: read-var ga#7140:u32
        let s_9_1: u32 = fn_state.ga_7140;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: not s_9_2
        let s_9_3: bool = !s_9_2;
        // N s_9_4: branch s_9_3 b14 b10
        if s_9_3 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #90704u : u32
        let s_10_0: u32 = 90704;
        // D s_10_1: read-reg s_10_0:struct
        let s_10_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: call _get_SCR_EL3_Type_GPF(s_10_1)
        let s_10_2: bool = u_get_SCR_EL3_Type_GPF(state, tracer, s_10_1);
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // C s_10_4: const #1u : u8
        let s_10_4: bool = true;
        // C s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // D s_10_6: cmp-eq s_10_3 s_10_5
        let s_10_6: bool = ((s_10_3) == (s_10_5));
        // N s_10_7: branch s_10_6 b13 b11
        if s_10_6 {
            return block_13(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#9887 <= s_11_0
        fn_state.gs_9887 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var gs#9887:u8
        let s_12_0: bool = fn_state.gs_9887;
        // D s_12_1: write-var return_value <= s_12_0
        fn_state.return_value = s_12_0;
        // N s_12_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #16975u : u32
        let s_13_0: u32 = 16975;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 2u16);
        // C s_13_3: const #424u : u32
        let s_13_3: u32 = 424;
        // D s_13_4: read-reg s_13_3:u8
        let s_13_4: u8 = {
            let value = state.read_register::<u8>(s_13_3 as isize);
            tracer.read_register(s_13_3 as isize, value);
            value
        };
        // D s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 2u16);
        // D s_13_6: cmp-ne s_13_2 s_13_5
        let s_13_6: bool = ((s_13_2) != (s_13_5));
        // D s_13_7: write-var gs#9887 <= s_13_6
        fn_state.gs_9887 = s_13_6;
        // N s_13_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var ga#7145:u8
        let s_14_0: bool = fn_state.ga_7145;
        // D s_14_1: write-var return_value <= s_14_0
        fn_state.return_value = s_14_0;
        // N s_14_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#9879 <= s_15_0
        fn_state.gs_9879 = s_15_0;
        // N s_15_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}

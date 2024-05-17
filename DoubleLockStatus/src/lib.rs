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
use u_get_DBGOSDLR_Type_DLK::*;
use u_get_DBGPRCR_EL1_Type_CORENPDRQ::*;
use u_get_OSDLR_EL1_Type_DLK::*;
use DBGOSDLR_read::*;
use u_get_DBGPRCR_Type_CORENPDRQ::*;
use ELUsingAArch32::*;
use Halted::*;
use DBGPRCR_read::*;
use HaveDoubleLock::*;
use common::*;
pub fn DoubleLockStatus<T: Tracer>(state: &mut State, tracer: &T, gs_4673: ()) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_4675: bool,
        gs_4676: bool,
        gs_4674: bool,
        gs_4677: bool,
        return_value: bool,
        gs_4673: (),
    }
    let fn_state = FunctionState {
        gs_4673,
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
        // S s_0_1: call HaveDoubleLock(s_0_0)
        let s_0_1: bool = HaveDoubleLock(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b17 b1
        if s_0_2 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #440u : u32
        let s_1_0: u32 = 440;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call ELUsingAArch32(s_1_1)
        let s_1_2: bool = ELUsingAArch32(state, tracer, s_1_1);
        // N s_1_3: branch s_1_2 b10 b2
        if s_1_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #102856u : u32
        let s_2_0: u32 = 102856;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_OSDLR_EL1_Type_DLK(s_2_1)
        let s_2_2: bool = u_get_OSDLR_EL1_Type_DLK(state, tracer, s_2_1);
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: const #1u : u8
        let s_2_4: bool = true;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // N s_2_7: branch s_2_6 b9 b3
        if s_2_6 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#4674 <= s_3_0
        fn_state.gs_4674 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#4674:u8
        let s_4_0: bool = fn_state.gs_4674;
        // N s_4_1: branch s_4_0 b8 b5
        if s_4_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#4675 <= s_5_0
        fn_state.gs_4675 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var gs#4675:u8
        let s_6_0: bool = fn_state.gs_4675;
        // D s_6_1: write-var return_value <= s_6_0
        fn_state.return_value = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var return_value:u8
        let s_7_0: bool = fn_state.return_value;
        // N s_7_1: return s_7_0
        return s_7_0;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call Halted(s_8_0)
        let s_8_1: bool = Halted(state, tracer, s_8_0);
        // S s_8_2: not s_8_1
        let s_8_2: bool = !s_8_1;
        // D s_8_3: write-var gs#4675 <= s_8_2
        fn_state.gs_4675 = s_8_2;
        // N s_8_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #90304u : u32
        let s_9_0: u32 = 90304;
        // D s_9_1: read-reg s_9_0:struct
        let s_9_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call _get_DBGPRCR_EL1_Type_CORENPDRQ(s_9_1)
        let s_9_2: bool = u_get_DBGPRCR_EL1_Type_CORENPDRQ(state, tracer, s_9_1);
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // C s_9_4: const #0u : u8
        let s_9_4: bool = false;
        // C s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 1u16);
        // D s_9_6: cmp-eq s_9_3 s_9_5
        let s_9_6: bool = ((s_9_3) == (s_9_5));
        // D s_9_7: write-var gs#4674 <= s_9_6
        fn_state.gs_4674 = s_9_6;
        // N s_9_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call DBGOSDLR_read(s_10_0)
        let s_10_1: ProductType700c18a878c5601b = DBGOSDLR_read(state, tracer, s_10_0);
        // S s_10_2: call _get_DBGOSDLR_Type_DLK(s_10_1)
        let s_10_2: bool = u_get_DBGOSDLR_Type_DLK(state, tracer, s_10_1);
        // S s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // C s_10_4: const #1u : u8
        let s_10_4: bool = true;
        // C s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // S s_10_6: cmp-eq s_10_3 s_10_5
        let s_10_6: bool = ((s_10_3) == (s_10_5));
        // N s_10_7: branch s_10_6 b16 b11
        if s_10_6 {
            return block_16(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#4676 <= s_11_0
        fn_state.gs_4676 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var gs#4676:u8
        let s_12_0: bool = fn_state.gs_4676;
        // N s_12_1: branch s_12_0 b15 b13
        if s_12_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#4677 <= s_13_0
        fn_state.gs_4677 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var gs#4677:u8
        let s_14_0: bool = fn_state.gs_4677;
        // D s_14_1: write-var return_value <= s_14_0
        fn_state.return_value = s_14_0;
        // N s_14_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call Halted(s_15_0)
        let s_15_1: bool = Halted(state, tracer, s_15_0);
        // S s_15_2: not s_15_1
        let s_15_2: bool = !s_15_1;
        // D s_15_3: write-var gs#4677 <= s_15_2
        fn_state.gs_4677 = s_15_2;
        // N s_15_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call DBGPRCR_read(s_16_0)
        let s_16_1: ProductType700c18a878c5601b = DBGPRCR_read(state, tracer, s_16_0);
        // S s_16_2: call _get_DBGPRCR_Type_CORENPDRQ(s_16_1)
        let s_16_2: bool = u_get_DBGPRCR_Type_CORENPDRQ(state, tracer, s_16_1);
        // S s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // C s_16_4: const #0u : u8
        let s_16_4: bool = false;
        // C s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // S s_16_6: cmp-eq s_16_3 s_16_5
        let s_16_6: bool = ((s_16_3) == (s_16_5));
        // D s_16_7: write-var gs#4676 <= s_16_6
        fn_state.gs_4676 = s_16_6;
        // N s_16_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var return_value <= s_17_0
        fn_state.return_value = s_17_0;
        // N s_17_2: jump b7
        return block_7(state, tracer, fn_state);
    }
}

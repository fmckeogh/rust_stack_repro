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
use ThisInstrAddr::*;
use CurrentSecurityState::*;
use ELUsingAArch32::*;
use AArch32_EnterHypMode::*;
use common::*;
pub fn AArch32_TakeHypTrapException__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    except: ProductTypeb7f99f96751e17c4,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_30746: bool,
        gs_30745: bool,
        except: ProductTypeb7f99f96751e17c4,
    }
    let fn_state = FunctionState {
        except,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #432u : u32
        let s_0_0: u32 = 432;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #2u : u8
        let s_0_2: u8 = 2;
        // D s_0_3: cmp-lt s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) < (s_0_2));
        // N s_0_4: branch s_0_3 b6 b1
        if s_0_3 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#30745 <= s_1_0
        fn_state.gs_30745 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#30745:u8
        let s_2_0: bool = fn_state.gs_30745;
        // N s_2_1: branch s_2_0 b5 b3
        if s_2_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#30746 <= s_3_0
        fn_state.gs_30746 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#30746:u8
        let s_4_0: bool = fn_state.gs_30746;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #32s : i64
        let s_4_2: i64 = 32;
        // C s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // S s_4_4: call ThisInstrAddr(s_4_3)
        let s_4_4: Bits = ThisInstrAddr(state, tracer, s_4_3);
        // S s_4_5: cast reint s_4_4 -> u32
        let s_4_5: u32 = (s_4_4.value() as u32);
        // C s_4_6: const #20u : u8
        let s_4_6: u8 = 20;
        // C s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 8u16);
        // C s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (s_4_7.value() as i128);
        // C s_4_9: cast reint s_4_8 -> i64
        let s_4_9: i64 = (s_4_8 as i64);
        // C s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (i128::try_from(s_4_9).unwrap());
        // D s_4_11: read-var except:struct
        let s_4_11: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_4_12: call AArch32_EnterHypMode(s_4_11, s_4_5, s_4_10)
        let s_4_12: () = AArch32_EnterHypMode(state, tracer, s_4_11, s_4_5, s_4_10);
        // N s_4_13: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #432u : u32
        let s_5_0: u32 = 432;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call ELUsingAArch32(s_5_1)
        let s_5_2: bool = ELUsingAArch32(state, tracer, s_5_1);
        // D s_5_3: write-var gs#30746 <= s_5_2
        fn_state.gs_30746 = s_5_2;
        // N s_5_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call CurrentSecurityState(s_6_0)
        let s_6_1: u32 = CurrentSecurityState(state, tracer, s_6_0);
        // C s_6_2: const #0u : u32
        let s_6_2: u32 = 0;
        // S s_6_3: cmp-eq s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) == (s_6_2));
        // D s_6_4: write-var gs#30745 <= s_6_3
        fn_state.gs_30745 = s_6_3;
        // N s_6_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
